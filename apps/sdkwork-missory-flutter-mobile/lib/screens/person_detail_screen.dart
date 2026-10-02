import 'package:flutter/material.dart';

import 'package:sdkwork_missory_app_sdk/sdkwork_missory_app_sdk.dart' as sdk;
import 'package:sdkwork_missory_flutter_mobile_core/sdkwork_missory_flutter_mobile_core.dart';

/// Aggregated detail payload for one person: retrieve + timeline.
class PersonDetailData {
  PersonDetailData({this.detail, this.timeline = const []});

  final sdk.MissoryPersonDetail? detail;
  final List<sdk.MissoryTimelineEntry> timeline;
}

class PersonDetailScreen extends StatefulWidget {
  const PersonDetailScreen({super.key, required this.runtime, this.personId});

  final MissoryRuntime runtime;
  final String? personId;

  @override
  State<PersonDetailScreen> createState() => _PersonDetailScreenState();
}

class _PersonDetailScreenState extends State<PersonDetailScreen> {
  late Future<PersonDetailData> _future;
  sdk.MissoryPerson? _person;
  String _briefing = '';
  List<String> _briefingTopics = [];
  bool _briefingBusy = false;

  String get _personId => widget.personId ?? '';

  @override
  void initState() {
    super.initState();
    _future = _load();
  }

  Future<PersonDetailData> _load() async {
    final detail = await widget.runtime.services.person(_personId);
    final timeline = await widget.runtime.services.personTimeline(_personId);
    _person = detail?.person;
    return PersonDetailData(detail: detail, timeline: timeline);
  }

  Future<void> _generateBriefing() async {
    if (_briefingBusy) return;
    setState(() => _briefingBusy = true);
    try {
      final result = await widget.runtime.services.briefing(_personId);
      if (!mounted) return;
      setState(() {
        _briefing = result?.briefing ?? '';
        _briefingTopics = result?.topics ?? const <String>[];
      });
    } finally {
      if (mounted) setState(() => _briefingBusy = false);
    }
  }

  Future<void> _editPerson() async {
    final person = _person;
    if (person == null) return;
    final updated = await _showEditDialog(person);
    if (updated != true) return;
    setState(() => _future = _load());
  }

  Future<bool?> _showEditDialog(sdk.MissoryPerson person) {
    final displayName = TextEditingController(text: person.displayName ?? '');
    final title = TextEditingController(text: person.title ?? '');
    final company = TextEditingController(text: person.company ?? '');
    final city = TextEditingController(text: person.city ?? '');
    final birthday = TextEditingController(text: person.birthday ?? '');
    final bio = TextEditingController(text: person.bio ?? '');
    return showDialog<bool>(
      context: context,
      builder: (context) => AlertDialog(
        title: const Text('编辑人物'),
        content: SizedBox(
          width: 320,
          child: ListView(
            shrinkWrap: true,
            children: [
              TextField(controller: displayName, decoration: const InputDecoration(labelText: '姓名')),
              TextField(controller: title, decoration: const InputDecoration(labelText: '称谓')),
              TextField(controller: company, decoration: const InputDecoration(labelText: '公司')),
              TextField(controller: city, decoration: const InputDecoration(labelText: '城市')),
              TextField(controller: birthday, decoration: const InputDecoration(labelText: '生日（如 05-20）')),
              TextField(controller: bio, decoration: const InputDecoration(labelText: '备注')),
            ],
          ),
        ),
        actions: [
          TextButton(onPressed: () => Navigator.pop(context), child: const Text('取消')),
          TextButton(
            onPressed: () async {
              if (displayName.text.trim().isEmpty) return;
              await widget.runtime.services.updatePerson(
                _personId,
                sdk.MissoryPersonUpsertRequest(
                  displayName: displayName.text.trim(),
                  title: title.text.trim(),
                  company: company.text.trim(),
                  city: city.text.trim(),
                  birthday: birthday.text.trim(),
                  bio: bio.text.trim(),
                ),
              );
              if (context.mounted) Navigator.pop(context, true);
            },
            child: const Text('保存'),
          ),
        ],
      ),
    );
  }

  Future<void> _deletePerson() async {
    final confirmed = await showDialog<bool>(
      context: context,
      builder: (context) => AlertDialog(
        title: const Text('删除人物'),
        content: const Text('将同时删除 TA 的关系与记忆关联，确定删除？'),
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
    await widget.runtime.services.deletePerson(_personId);
    if (!mounted) return;
    Navigator.of(context).pop();
  }

  @override
  Widget build(BuildContext context) {
    if (_personId.isEmpty) {
      return Scaffold(
        appBar: AppBar(title: const Text('人物详情')),
        body: const Center(child: Text('缺少人物 ID')),
      );
    }
    return Scaffold(
      appBar: AppBar(
        title: const Text('人物详情'),
        actions: [
          IconButton(tooltip: '编辑', icon: const Icon(Icons.edit_outlined), onPressed: _editPerson),
          IconButton(tooltip: '删除', icon: const Icon(Icons.delete_outline), onPressed: _deletePerson),
        ],
      ),
      body: FutureBuilder<PersonDetailData>(
        future: _future,
        builder: (context, snapshot) {
          if (snapshot.connectionState != ConnectionState.done) {
            return const Center(child: CircularProgressIndicator());
          }
          if (snapshot.hasError) {
            return Center(child: Text('加载失败：${snapshot.error}'));
          }
          final data = snapshot.data;
          final detail = data?.detail;
          final person = detail?.person;
          if (person == null) {
            return const Center(child: Text('未找到该人物'));
          }
          return ListView(
            padding: const EdgeInsets.all(12),
            children: [
              Text(
                person.displayName ?? '',
                style: Theme.of(context).textTheme.headlineSmall,
              ),
              Text(_personSubtitle(person)),
              if ((person.bio ?? '').isNotEmpty) Padding(padding: const EdgeInsets.only(top: 8), child: Text(person.bio ?? '')),
              const SizedBox(height: 12),
              FilledButton.icon(
                onPressed: _briefingBusy ? null : _generateBriefing,
                icon: const Icon(Icons.assistant),
                label: const Text('生成见面简报'),
              ),
              if (_briefing.isNotEmpty)
                Padding(
                  padding: const EdgeInsets.only(top: 8),
                  child: Column(
                    crossAxisAlignment: CrossAxisAlignment.start,
                    children: [
                      const Text('见面简报', style: TextStyle(fontWeight: FontWeight.bold)),
                      Text(_briefing),
                      if (_briefingTopics.isNotEmpty)
                        Padding(
                          padding: const EdgeInsets.only(top: 4),
                          child: Text('话题：${_briefingTopics.join('、')}'),
                        ),
                    ],
                  ),
                ),
              const SizedBox(height: 12),
              const Text('关系', style: TextStyle(fontWeight: FontWeight.bold)),
              for (final relationship in detail?.relationships ?? const <sdk.MissoryRelationship>[])
                ListTile(
                  contentPadding: EdgeInsets.zero,
                  title: Text((relationship.relationshipTypes ?? const <String>[]).join(' / ')),
                  subtitle: Text(_relationshipSubtitle(relationship)),
                ),
              if (detail?.relationships == null || detail!.relationships!.isEmpty)
                const Padding(padding: EdgeInsets.only(bottom: 8), child: Text('暂无关系记录')),
              const SizedBox(height: 8),
              const Text('时间线', style: TextStyle(fontWeight: FontWeight.bold)),
              for (final entry in data?.timeline ?? const <sdk.MissoryTimelineEntry>[])
                ListTile(
                  contentPadding: EdgeInsets.zero,
                  leading: const Icon(Icons.timeline),
                  title: Text(entry.title ?? ''),
                  subtitle: Text(
                    [entry.occurredAt ?? '', entry.detail ?? ''].where((part) => part.isNotEmpty).join(' · '),
                  ),
                ),
              if (data?.timeline.isEmpty ?? true) const Padding(padding: EdgeInsets.only(bottom: 8), child: Text('暂无时间线')),
              const SizedBox(height: 8),
              const Text('最近记忆', style: TextStyle(fontWeight: FontWeight.bold)),
              for (final memory in detail?.recentMemories ?? const <sdk.MissoryMemory>[])
                ListTile(
                  contentPadding: EdgeInsets.zero,
                  title: Text(memory.content ?? ''),
                  subtitle: Text('${memory.type ?? ''} · ${memory.status ?? ''}'),
                ),
              if (detail?.recentMemories == null || detail!.recentMemories!.isEmpty)
                const Text('暂无最近记忆'),
            ],
          );
        },
      ),
    );
  }

  String _personSubtitle(sdk.MissoryPerson person) {
    return [
      person.title ?? '',
      person.company ?? '',
      person.city ?? '',
      if ((person.birthday ?? '').isNotEmpty) '生日 ${person.birthday}',
    ].where((part) => part.isNotEmpty).join(' · ');
  }

  String _relationshipSubtitle(sdk.MissoryRelationship relationship) {
    return [
      relationship.description ?? '',
      if (relationship.importance != null && relationship.importance!.isNotEmpty)
        '重要程度 ${relationship.importance}',
      if ((relationship.lastContactedAt ?? '').isNotEmpty)
        '最近联系 ${relationship.lastContactedAt}',
    ].where((part) => part.isNotEmpty).join(' · ');
  }
}
