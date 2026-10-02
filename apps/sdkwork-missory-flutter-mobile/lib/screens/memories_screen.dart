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

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      appBar: AppBar(title: const Text('记忆')),
      bottomNavigationBar: const MissoryNavBar(currentIndex: 2),
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
                          trailing: row.status == 'candidate'
                              ? Row(
                                  mainAxisSize: MainAxisSize.min,
                                  children: [
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
                                )
                              : null,
                        ),
                    ],
                  ),
          ),
        ],
      ),
    );
  }
}
