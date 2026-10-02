import 'package:flutter/material.dart';

import '../app.dart';
import 'package:sdkwork_missory_app_sdk/sdkwork_missory_app_sdk.dart' as sdk;
import 'package:sdkwork_missory_flutter_mobile_core/sdkwork_missory_flutter_mobile_core.dart';

class HomeRow {
  HomeRow({required this.title, required this.subtitle, this.reminderId});

  final String title;
  final String subtitle;
  final String? reminderId;
}

class HomeScreen extends StatefulWidget {
  const HomeScreen({super.key, required this.runtime, this.onSessionEnded});

  final MissoryRuntime runtime;
  final VoidCallback? onSessionEnded;

  @override
  State<HomeScreen> createState() => _HomeScreenState();
}

class _HomeScreenState extends State<HomeScreen> {
  late Future<List<HomeRow>> _future;

  @override
  void initState() {
    super.initState();
    _future = _load();
  }

  Future<List<HomeRow>> _load() async {
    final digest = await widget.runtime.services.homeToday();
    final reminders = digest?.todayReminders ?? const <sdk.MissoryReminder>[];
    return [
      for (final reminder in reminders)
        HomeRow(
          title: '${reminder.personName ?? ''} ${reminder.title ?? ''}',
          subtitle: reminder.reminderId ?? '',
          reminderId: reminder.reminderId,
        ),
    ];
  }

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      appBar: AppBar(
        title: const Text('今天，有谁值得你想起？'),
        actions: [
          IconButton(
            tooltip: '退出登录',
            icon: const Icon(Icons.logout_outlined),
            onPressed: () async {
              await widget.runtime.session.logout();
              widget.onSessionEnded?.call();
            },
          ),
        ],
      ),
      bottomNavigationBar: MissoryNavBar(currentIndex: 0),
      body: FutureBuilder<List<HomeRow>>(
        future: _future,
        builder: (context, snapshot) {
          if (snapshot.connectionState != ConnectionState.done) {
            return const Center(child: CircularProgressIndicator());
          }
          if (snapshot.hasError) {
            return Center(child: Text('加载失败：${snapshot.error}'));
          }
          final rows = snapshot.data ?? const <HomeRow>[];
          if (rows.isEmpty) {
            return const Center(
              child: Text('暂无提醒，去「人物」页创建第一个重要的人。'),
            );
          }
          return ListView(
            children: [
              for (final row in rows)
                ListTile(
                  title: Text(row.title),
                  subtitle: Text(row.subtitle),
                  trailing: Row(
                    mainAxisSize: MainAxisSize.min,
                    children: [
                      TextButton(
                        onPressed: () async {
                          await widget.runtime.services.snoozeReminder(row.reminderId ?? '');
                          setState(() => _future = _load());
                        },
                        child: const Text('稍后提醒'),
                      ),
                      TextButton(
                        onPressed: () async {
                          await widget.runtime.services.dismissReminder(row.reminderId ?? '');
                          setState(() => _future = _load());
                        },
                        child: const Text('忽略'),
                      ),
                    ],
                  ),
                ),
            ],
          );
        },
      ),
    );
  }
}
