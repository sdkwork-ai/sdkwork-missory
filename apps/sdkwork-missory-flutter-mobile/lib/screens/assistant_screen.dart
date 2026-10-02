import 'package:flutter/material.dart';

import '../app.dart';
import 'package:sdkwork_missory_app_sdk/sdkwork_missory_app_sdk.dart' as sdk;
import 'package:sdkwork_missory_flutter_mobile_core/sdkwork_missory_flutter_mobile_core.dart';

class AssistantScreen extends StatefulWidget {
  const AssistantScreen({super.key, required this.runtime});

  final MissoryRuntime runtime;

  @override
  State<AssistantScreen> createState() => _AssistantScreenState();
}

class _AssistantScreenState extends State<AssistantScreen> {
  final TextEditingController _question = TextEditingController();
  final TextEditingController _chat = TextEditingController();
  String _answer = '';
  String _draft = '';
  String _chatSummary = '';
  List<sdk.MissoryMemory> _chatCandidates = [];
  bool _busy = false;
  bool _summaryBusy = false;

  Future<void> _ask() async {
    if (_question.text.trim().isEmpty) return;
    setState(() => _busy = true);
    try {
      final answer = await widget.runtime.services.ask(_question.text);
      setState(() => _answer = answer?.answer ?? '');
    } finally {
      setState(() => _busy = false);
    }
  }

  Future<void> _makeDraft() async {
    final personId = await _promptPersonId(context);
    if (personId == null || personId.isEmpty) return;
    setState(() => _busy = true);
    try {
      final draft = await widget.runtime.services.draftBirthday(personId);
      setState(() => _draft = '${draft?.draft ?? ''}\n\n${draft?.disclaimer ?? ''}');
    } finally {
      setState(() => _busy = false);
    }
  }

  /// Pastes chat text into the chat-summaries endpoint; extracted memories are
  /// rendered read-only candidates — confirming them happens on the memories
  /// screen, and nothing is ever sent automatically.
  Future<void> _summarizeChat() async {
    if (_chat.text.trim().isEmpty || _summaryBusy) return;
    setState(() => _summaryBusy = true);
    try {
      final summary = await widget.runtime.services.chatSummary(_chat.text);
      if (!mounted) return;
      setState(() {
        _chatSummary = summary?.summary ?? '';
        _chatCandidates = summary?.candidateMemories ?? const <sdk.MissoryMemory>[];
      });
    } finally {
      setState(() => _summaryBusy = false);
    }
  }

  Future<String?> _promptPersonId(BuildContext context) {
    final controller = TextEditingController();
    return showDialog<String>(
      context: context,
      builder: (context) => AlertDialog(
        title: const Text('人物 ID'),
        content: TextField(controller: controller),
        actions: [
          TextButton(onPressed: () => Navigator.pop(context), child: const Text('取消')),
          TextButton(
            onPressed: () => Navigator.pop(context, controller.text),
            child: const Text('确定'),
          ),
        ],
      ),
    );
  }

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      appBar: AppBar(title: const Text('AI 社交助手')),
      bottomNavigationBar: const MissoryNavBar(currentIndex: 4),
      body: SingleChildScrollView(
        padding: const EdgeInsets.all(12),
        child: Column(
          crossAxisAlignment: CrossAxisAlignment.start,
          children: [
            Row(
              children: [
                Expanded(
                  child: TextField(
                    controller: _question,
                    decoration: const InputDecoration(hintText: '李明是谁？'),
                  ),
                ),
                IconButton(onPressed: _busy ? null : _ask, icon: const Icon(Icons.send)),
              ],
            ),
            if (_answer.isNotEmpty)
              Padding(padding: const EdgeInsets.all(8), child: Text(_answer)),
            const SizedBox(height: 12),
            FilledButton(onPressed: _busy ? null : _makeDraft, child: const Text('生成生日祝福草稿')),
            if (_draft.isNotEmpty)
              Padding(padding: const EdgeInsets.all(8), child: Text(_draft)),
            const SizedBox(height: 12),
            const Text('对话汇总', style: TextStyle(fontWeight: FontWeight.bold)),
            const Text('粘贴一段聊天记录，AI 提炼摘要与候选记忆（仅供参考，不会自动保存或发送）。'),
            const SizedBox(height: 8),
            TextField(
              controller: _chat,
              decoration: const InputDecoration(hintText: '粘贴对话文本…'),
              maxLines: 4,
            ),
            const SizedBox(height: 8),
            FilledButton.icon(
              onPressed: _summaryBusy ? null : _summarizeChat,
              icon: const Icon(Icons.summarize),
              label: const Text('汇总对话'),
            ),
            if (_chatSummary.isNotEmpty)
              Padding(
                padding: const EdgeInsets.all(8),
                child: Column(
                  crossAxisAlignment: CrossAxisAlignment.start,
                  children: [
                    const Text('摘要', style: TextStyle(fontWeight: FontWeight.bold)),
                    Text(_chatSummary),
                    const SizedBox(height: 8),
                    const Text('候选记忆', style: TextStyle(fontWeight: FontWeight.bold)),
                    for (final candidate in _chatCandidates)
                      Padding(
                        padding: const EdgeInsets.only(top: 4),
                        child: Text('· ${candidate.content ?? ''}'),
                      ),
                    if (_chatCandidates.isEmpty) const Text('未提取到候选记忆'),
                  ],
                ),
              ),
          ],
        ),
      ),
    );
  }
}
