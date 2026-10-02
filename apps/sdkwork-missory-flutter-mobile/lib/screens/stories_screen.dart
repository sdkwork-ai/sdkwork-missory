import 'package:flutter/material.dart';

import '../app.dart';
import 'package:sdkwork_missory_flutter_mobile_core/sdkwork_missory_flutter_mobile_core.dart';

class StoryRow {
  StoryRow({
    required this.id,
    required this.title,
    required this.summary,
    required this.location,
    required this.startedAt,
    required this.endedAt,
  });

  final String id;
  final String title;
  final String summary;
  final String location;
  final String startedAt;
  final String endedAt;
}

class StoriesScreen extends StatefulWidget {
  const StoriesScreen({super.key, required this.runtime});

  final MissoryRuntime runtime;

  @override
  State<StoriesScreen> createState() => _StoriesScreenState();
}

class _StoriesScreenState extends State<StoriesScreen> {
  List<StoryRow> _rows = [];
  bool _loading = true;
  String? _error;

  @override
  void initState() {
    super.initState();
    _refresh();
  }

  Future<void> _refresh() async {
    try {
      final stories = await widget.runtime.services.stories();
      if (!mounted) return;
      setState(() {
        _rows = [
          for (final story in stories)
            StoryRow(
              id: story.id ?? '',
              title: story.title ?? '',
              summary: story.summary ?? '',
              location: story.location ?? '',
              startedAt: story.startedAt ?? '',
              endedAt: story.endedAt ?? '',
            ),
        ];
        _loading = false;
        _error = null;
      });
    } catch (error) {
      if (!mounted) return;
      setState(() {
        _loading = false;
        _error = '$error';
      });
    }
  }

  Future<void> _create() async {
    final created = await _showCreateDialog();
    if (created != true) return;
    await _refresh();
  }

  Future<bool?> _showCreateDialog() {
    final title = TextEditingController();
    final location = TextEditingController();
    final startedAt = TextEditingController();
    final endedAt = TextEditingController();
    return showDialog<bool>(
      context: context,
      builder: (context) => AlertDialog(
        title: const Text('创建故事'),
        content: SizedBox(
          width: 320,
          child: ListView(
            shrinkWrap: true,
            children: [
              TextField(controller: title, decoration: const InputDecoration(labelText: '标题')),
              TextField(controller: location, decoration: const InputDecoration(labelText: '地点（可选）')),
              TextField(
                controller: startedAt,
                decoration: const InputDecoration(labelText: '开始时间（可选，ISO 格式）'),
              ),
              TextField(
                controller: endedAt,
                decoration: const InputDecoration(labelText: '结束时间（可选，ISO 格式）'),
              ),
            ],
          ),
        ),
        actions: [
          TextButton(onPressed: () => Navigator.pop(context), child: const Text('取消')),
          TextButton(
            onPressed: () async {
              if (title.text.trim().isEmpty) return;
              await widget.runtime.services.createStory(
                title: title.text.trim(),
                location: location.text,
                startedAt: startedAt.text,
                endedAt: endedAt.text,
              );
              if (context.mounted) Navigator.pop(context, true);
            },
            child: const Text('创建'),
          ),
        ],
      ),
    );
  }

  Future<void> _summarize(StoryRow row) async {
    await widget.runtime.services.summarizeStory(row.id);
    if (!mounted) return;
    ScaffoldMessenger.of(context).showSnackBar(
      const SnackBar(content: Text('已生成故事摘要')),
    );
    await _refresh();
  }

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      appBar: AppBar(title: const Text('故事')),
      bottomNavigationBar: const MissoryNavBar(currentIndex: 3),
      floatingActionButton: FloatingActionButton(
        onPressed: _create,
        child: const Icon(Icons.add),
      ),
      body: _loading
          ? const Center(child: CircularProgressIndicator())
          : _error != null
              ? Center(child: Text('加载失败：$_error'))
              : _rows.isEmpty
                  ? const Center(child: Text('暂无故事，点击右下角创建第一段共同经历。'))
                  : ListView(
                      children: [
                        for (final row in _rows)
                          ListTile(
                            title: Text(row.title),
                            subtitle: Text(
                              [
                                if (row.summary.isNotEmpty) row.summary,
                                if (row.location.isNotEmpty) row.location,
                                if (row.startedAt.isNotEmpty) row.startedAt,
                                if (row.endedAt.isNotEmpty) '～ ${row.endedAt}',
                              ].join(' · '),
                            ),
                            trailing: TextButton(
                              onPressed: () => _summarize(row),
                              child: const Text('AI 摘要'),
                            ),
                          ),
                      ],
                    ),
    );
  }
}
