import 'package:flutter/material.dart';

import '../app.dart';
import 'package:sdkwork_missory_flutter_mobile_core/sdkwork_missory_flutter_mobile_core.dart';

class AssistantScreen extends StatefulWidget {
  const AssistantScreen({super.key, required this.runtime});

  final MissoryRuntime runtime;

  @override
  State<AssistantScreen> createState() => _AssistantScreenState();
}

class _AssistantScreenState extends State<AssistantScreen> {
  final TextEditingController _question = TextEditingController();
  String _answer = '';
  String _draft = '';
  bool _busy = false;

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
      bottomNavigationBar: const MissoryNavBar(currentIndex: 3),
      body: Padding(
        padding: const EdgeInsets.all(12),
        child: Column(
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
          ],
        ),
      ),
    );
  }
}
