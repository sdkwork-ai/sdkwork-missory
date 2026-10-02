import 'package:flutter/material.dart';

import 'bootstrap/runtime.dart';
import 'screens/assistant_screen.dart';
import 'screens/home_screen.dart';
import 'screens/login_screen.dart';
import 'screens/memories_screen.dart';
import 'screens/people_screen.dart';

/// Session gate: development with the gateway bypass runs signed-in by
/// default (dev identity seeded in the client); every other environment
/// renders the credential-entry login until a dual-token session exists.
/// Stateful so login/logout transitions rebuild the console root.
class MissoryApp extends StatefulWidget {
  const MissoryApp({super.key, required this.runtime});

  final MissoryRuntime runtime;

  @override
  State<MissyApp> createState() => _MissoryAppState();
}

class _MissoryAppState extends State<MissyApp> {
  @override
  Widget build(BuildContext context) {
    final bypassed = widget.runtime.environment.environment == 'development';
    final authenticated = bypassed || widget.runtime.session.isAuthenticated;
    return MaterialApp(
      title: '念忆 · Missory',
      theme: ThemeData(colorSchemeSeed: const Color(0xFF0F766E), useMaterial3: true),
      home: authenticated
          ? HomeScreen(
              runtime: widget.runtime,
              onSessionEnded: () => setState(() {}),
            )
          : LoginScreen(
              runtime: widget.runtime,
              onSessionEstablished: () => setState(() {}),
            ),
      routes: authenticated
          ? {
              '/people': (context) => PeopleScreen(runtime: widget.runtime),
              '/memories': (context) => MemoriesScreen(runtime: widget.runtime),
              '/assistant': (context) => AssistantScreen(runtime: widget.runtime),
            }
          : const <String, WidgetBuilder>{},
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
