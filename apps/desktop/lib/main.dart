import 'package:flutter/material.dart';
import 'screens/feed_screen.dart';
import 'screens/explore_screen.dart';
import 'screens/groups_screen.dart';
import 'screens/pages_screen.dart';
import 'screens/profile_screen.dart';

void main() {
  runApp(const SovrApp());
}

class SovrApp extends StatefulWidget {
  const SovrApp({super.key});
  @override
  State<SovrApp> createState() => _SovrAppState();
}

class _SovrAppState extends State<SovrApp> {
  int _idx = 0;
  final _pages = const [
    FeedScreen(),
    ExploreScreen(),
    GroupsScreen(),
    PagesScreen(),
    ProfileScreen(),
  ];
  @override
  Widget build(BuildContext context) {
    return MaterialApp(
      title: 'SOVR',
      theme: ThemeData(useMaterial3: true),
      home: Scaffold(
        appBar: AppBar(title: const Text('SOVR')),
        body: _pages[_idx],
        bottomNavigationBar: NavigationBar(
          selectedIndex: _idx,
          onDestinationSelected: (i) => setState(() => _idx = i),
          destinations: const [
            NavigationDestination(icon: Icon(Icons.home_outlined), selectedIcon: Icon(Icons.home), label: 'Feed'),
            NavigationDestination(icon: Icon(Icons.search), label: 'Explore'),
            NavigationDestination(icon: Icon(Icons.forum_outlined), selectedIcon: Icon(Icons.forum), label: 'Groups'),
            NavigationDestination(icon: Icon(Icons.storefront_outlined), selectedIcon: Icon(Icons.storefront), label: 'Pages'),
            NavigationDestination(icon: Icon(Icons.person_outline), selectedIcon: Icon(Icons.person), label: 'Profile'),
          ],
        ),
      ),
    );
  }
}
