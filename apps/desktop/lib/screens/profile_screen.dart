import 'package:flutter/material.dart';
import '../services/sovr_api.dart';

class ProfileScreen extends StatefulWidget {
  const ProfileScreen({super.key});
  @override
  State<ProfileScreen> createState() => _ProfileScreenState();
}

class _ProfileScreenState extends State<ProfileScreen> {
  final _api = SovrApi.instance;
  String _ver = '…';
  @override
  void initState() { super.initState(); _api.version().then((v) => setState(() => _ver = v)); }
  @override
  Widget build(BuildContext context) {
    return Padding(
      padding: const EdgeInsets.all(24),
      child: Column(
        crossAxisAlignment: CrossAxisAlignment.start,
        children: [
          Text('SOVR $_ver', style: Theme.of(context).textTheme.headlineSmall),
          const SizedBox(height: 12),
          const Text('Profile settings go here (stub)'),
        ],
      ),
    );
  }
}
