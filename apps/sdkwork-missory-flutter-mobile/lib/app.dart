import 'package:flutter/material.dart';

import 'bootstrap/runtime.dart';
import 'screens/assistant_screen.dart';
import 'screens/home_screen.dart';
import 'screens/memories_screen.dart';
import 'screens/people_screen.dart';

class MissoryApp extends StatelessWidget {
  const MissoryApp({super.key, required this.runtime});

  final MissoryRuntime runtime;

  @override
  Widget build(BuildContext context) {
    return MaterialApp(
      title: '念忆 · Missory',
      theme: ThemeData(colorSchemeSeed: const Color(0xFF0F766E), useMaterial3: true),
      home: HomeScreen(runtime: runtime),
      routes: {
        '/people': (context) => PeopleScreen(runtime: runtime),
        '/memories': (context) => MemoriesScreen(runtime: runtime),
        '/assistant': (context) => AssistantScreen(runtime: runtime),
      },
    );
  }
}

/// Shared bottom navigation across the four P0 tabs.
class MissoryNavBar extends StatelessWidget {
  const MissoryNavBar({super.key, required this.currentIndex});

  final int currentIndex;

  @override
  Widget build(BuildContext context) {
    return NavigationBar(
      selectedIndex: currentIndex,
      destinations: const [
        NavigationDestination(icon: Icon(Icons.home_outlined), label: '首页'),
        NavigationDestination(icon: Icon(Icons.people_outline), label: '人物'),
        NavigationDestination(icon: Icon(Icons.auto_stories_outlined), label: '记忆'),
        NavigationDestination(icon: Icon(Icons.smart_toy_outlined), label: 'AI'),
      ],
      onDestinationSelected: (index) {
        const routes = ['/people', '/memories', '/assistant'];
        if (index == 0) {
          Navigator.of(context).popUntil((route) => route.isFirst);
        } else {
          Navigator.of(context).pushNamed(routes[index - 1]);
        }
      },
    );
  }
}
