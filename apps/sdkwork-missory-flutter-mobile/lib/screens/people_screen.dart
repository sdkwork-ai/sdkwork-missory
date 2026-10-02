import 'package:flutter/material.dart';

import '../app.dart';
import 'package:sdkwork_missory_flutter_mobile_core/sdkwork_missory_flutter_mobile_core.dart';

class PeopleRow {
  PeopleRow({required this.id, required this.name, required this.title});

  final String id;
  final String name;
  final String title;
}

class PeopleScreen extends StatefulWidget {
  const PeopleScreen({super.key, required this.runtime});

  final MissoryRuntime runtime;

  @override
  State<PeopleScreen> createState() => _PeopleScreenState();
}

class _PeopleScreenState extends State<PeopleScreen> {
  late Future<List<PeopleRow>> _future;

  @override
  void initState() {
    super.initState();
    _future = _load();
  }

  Future<List<PeopleRow>> _load() async {
    final people = await widget.runtime.services.people();
    return [
      for (final person in people)
        PeopleRow(
          id: person.id ?? '',
          name: person.displayName ?? '',
          title: person.title ?? '',
        ),
    ];
  }

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      appBar: AppBar(title: const Text('人物')),
      bottomNavigationBar: const MissoryNavBar(currentIndex: 1),
      floatingActionButton: FloatingActionButton(
        onPressed: () async {
          final name = await _promptName(context);
          if (name != null && name.trim().isNotEmpty) {
            await widget.runtime.services.createPerson(name.trim());
            setState(() => _future = _load());
          }
        },
        child: const Icon(Icons.person_add),
      ),
      body: FutureBuilder<List<PeopleRow>>(
        future: _future,
        builder: (context, snapshot) {
          if (snapshot.connectionState != ConnectionState.done) {
            return const Center(child: CircularProgressIndicator());
          }
          final rows = snapshot.data ?? const <PeopleRow>[];
          return ListView(
            children: [
              for (final row in rows)
                ListTile(title: Text(row.name), subtitle: Text(row.title)),
            ],
          );
        },
      ),
    );
  }

  Future<String?> _promptName(BuildContext context) {
    final controller = TextEditingController();
    return showDialog<String>(
      context: context,
      builder: (context) => AlertDialog(
        title: const Text('创建人物'),
        content: TextField(
          controller: controller,
          decoration: const InputDecoration(hintText: '姓名'),
        ),
        actions: [
          TextButton(onPressed: () => Navigator.pop(context), child: const Text('取消')),
          TextButton(
            onPressed: () => Navigator.pop(context, controller.text),
            child: const Text('创建'),
          ),
        ],
      ),
    );
  }
}
