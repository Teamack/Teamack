import 'package:flutter/material.dart';
import 'screens/feed_screen.dart';
import 'screens/explore_screen.dart';
import 'screens/groups_screen.dart';
import 'screens/pages_screen.dart';
import 'screens/profile_screen.dart';

void main() => runApp(const SovrMobile());

class SovrMobile extends StatelessWidget {
  const SovrMobile({super.key});
  @override
  Widget build(BuildContext context) => const MaterialApp(
    debugShowCheckedModeBanner: false,
    home: FeedScreen(),
  );
}
