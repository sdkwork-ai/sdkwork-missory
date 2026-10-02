import 'package:flutter/material.dart';

import '../app.dart';
import 'package:sdkwork_missory_flutter_mobile_core/sdkwork_missory_flutter_mobile_core.dart';

class MemoryRow {
  MemoryRow({
    required this.id,
    required this.content,
    required this.type,
    required this.origin,
    required this.status,
  });

  final String id;
  final String content;
  final String type;
  final String origin;
  final String status;
}

/// Memory types per PRD §17 (MissoryMemoryType enum).
const Map<String, String> kMemoryTypeLabels = {
  'semantic': '事实',
  'episodic': '经历',
  'temporal': '时间',
  'relationship': '关系',
  'preference': '偏好',
  'commitment': '承诺',
};

class MemoriesScreen extends StatefulWidget {
  const MemoriesScreen({super.key, required this.runtime});

  final MissoryRuntime runtime;

  @override
  State<MemoriesScreen> createState() => _MemoriesScreenState();
}

class _MemoriesScreenState extends State<MemoriesScreen> {
  final TextEditingController _text = TextEditingController();
  List<MemoryRow> _rows = [];
  bool _loading = true;

  @override
  void initState() {
    super.initState();
    _refresh();
  }

  Future<void> _refresh() async {
    final memories = await widget.runtime.services.memories();
    setState(() {
      _rows = [
        for (final memory in memories)
          MemoryRow(
            id: memory.id ?? '',
            content: memory.content ?? '',
            type: memory.type ?? '',
            origin: memory.origin ?? '',
            status: memory.status ?? '',
          ),
      ];
      _loading = false;
    });
  }

  Future<void> _extract() async {
    if (_text.text.trim().isEmpty) return;
    await widget.runtime.services.extractCandidates(_text.text);
    _text.clear();
    if (!mounted) return;
    ScaffoldMessenger.of(context).showSnackBar(
      const SnackBar(content: Text('已提取候选记忆，请在列表中确认')),
    );
    await _refresh();
  }

  Future<void> _createManual() async {
    final created = await _showCreateDialog();
    if (created != true) return;
    await _refresh();
  }

  Future<bool?> _showCreateDialog() {
    return showDialog<bool>(
      context: context,
      builder: (dialogContext) => FutureBuilder(
        future: widget.runtime.services.people(),
        builder: (context, snapshot) {
          if (snapshot.connectionState != ConnectionState.done) {
            return const AlertDialog(
              title: Text('记录一条记忆'),
              content: SizedBox(
                height: 64,
                child: Center(child: CircularProgressIndicator()),
              ),
            );
          }
          final people = snapshot.data ?? const [];
          String? personId = people.isNotEmpty ? (people.first.id ?? '') : null;
          String type = kMemoryTypeLabels.keys.first;
          final content = TextEditingController();
          return StatefulBuilder(
            builder: (context, setDialogState) => AlertDialog(
              title: const Text('记录一条记忆'),
              content: SizedBox(
                width: 320,
                child: people.isEmpty
                    ? const Text('先在「人物」页创建人物，再记录记忆。')
                    : ListView(
                        shrinkWrap: true,
                        children: [
                          DropdownButtonFormField<String>(
                            initialValue: personId,
                            decoration: const InputDecoration(labelText: '人物'),
                            items: [
                              for (final person in people)
                                DropdownMenuItem(
                                  value: person.id ?? '',
                                  child: Text(person.displayName ?? ''),
                                ),
                            ],
                            onChanged: (value) => setDialogState(() => personId = value),
                          ),
                          DropdownButtonFormField<String>(
                            initialValue: type,
                            decoration: const InputDecoration(labelText: '类型'),
                            items: [
                              for (final entry in kMemoryTypeLabels.entries)
                                DropdownMenuItem(value: entry.key, child: Text(entry.value)),
                            ],
                            onChanged: (value) => setDialogState(() => type = value ?? type),
                          ),
                          TextField(
                            controller: content,
                            decoration: const InputDecoration(labelText: '内容'),
                            maxLines: 3,
                          ),
                        ],
                      ),
              ),
              actions: [
                TextButton(onPressed: () => Navigator.pop(context), child: const Text('取消')),
                TextButton(
                  onPressed: () async {
                    if (personId == null ||
                        personId!.isEmpty ||
                        content.text.trim().isEmpty) {
                      return;
                    }
                    await widget.runtime.services.createMemory(
                      personId: personId!,
                      type: type,
                      content: content.text.trim(),
                    );
                    if (dialogContext.mounted) Navigator.pop(dialogContext, true);
                  },
                  child: const Text('保存'),
                ),
              ],
            ),
          );
        },
      ),
    );
  }

  Future<void> _delete(MemoryRow row) async {
    final confirmed = await showDialog<bool>(
      context: context,
      builder: (context) => AlertDialog(
        title: const Text('删除记忆'),
        content: const Text('删除后不可恢复，确定删除这条记忆？'),
        actions: [
          TextButton(onPressed: () => Navigator.pop(context), child: const Text('取消')),
          TextButton(
            onPressed: () => Navigator.pop(context, true),
            child: const Text('删除'),
          ),
        ],
      ),
    );
    if (confirmed != true) return;
    await widget.runtime.services.deleteMemory(row.id);
    await _refresh();
  }

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      appBar: AppBar(title: const Text('记忆')),
      bottomNavigationBar: const MissoryNavBar(currentIndex: 2),
      floatingActionButton: FloatingActionButton(
        onPressed: _createManual,
        child: const Icon(Icons.add),
      ),
      body: Column(
        children: [
          Padding(
            padding: const EdgeInsets.all(12),
            child: Row(
              children: [
                Expanded(
                  child: TextField(
                    controller: _text,
                    decoration: const InputDecoration(hintText: '粘贴对话或随笔…'),
                  ),
                ),
                IconButton(onPressed: _extract, icon: const Icon(Icons.auto_awesome)),
              ],
            ),
          ),
          Expanded(
            child: _loading
                ? const Center(child: CircularProgressIndicator())
                : ListView(
                    children: [
                      for (final row in _rows)
                        ListTile(
                          title: Text(row.content),
                          subtitle: Text('${row.type} · ${row.origin} · ${row.status}'),
                          trailing: Row(
                            mainAxisSize: MainAxisSize.min,
                            children: [
                              if (row.status == 'candidate') ...[
                                TextButton(
                                  onPressed: () async {
                                    await widget.runtime.services.confirmMemory(row.id);
                                    await _refresh();
                                  },
                                  child: const Text('确认'),
                                ),
                                TextButton(
                                  onPressed: () async {
                                    await widget.runtime.services.rejectMemory(row.id);
                                    await _refresh();
                                  },
                                  child: const Text('拒绝'),
                                ),
                              ],
                              IconButton(
                                tooltip: '删除',
                                icon: const Icon(Icons.delete_outline),
                                onPressed: () => _delete(row),
                              ),
                            ],
                          ),
                        ),
                    ],
                  ),
          ),
        ],
      ),
    );
  }
}
