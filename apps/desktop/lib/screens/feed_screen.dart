import 'package:flutter/material.dart';
import '../services/sovr_api.dart';

class FeedScreen extends StatefulWidget {
  const FeedScreen({super.key});
  @override
  State<FeedScreen> createState() => _FeedScreenState();
}

class _FeedScreenState extends State<FeedScreen> {
  final _api = SovrApi.instance;
  List<PostView> _items = const [];
  bool _loading = true;

  @override
  void initState() {
    super.initState();
    _load();
  }

  Future<void> _load() async {
    final items = await _api.fetchTimeline();
    setState(() { _items = items; _loading = false; });
  }

  @override
  Widget build(BuildContext context) {
    return Stack(
      children: [
        if (_loading) const Center(child: CircularProgressIndicator()),
        if (!_loading)
          RefreshIndicator(
            onRefresh: _load,
            child: ListView.builder(
              itemCount: _items.length,
              itemBuilder: (_, i) {
                final it = _items[i];
                return ListTile(
                  title: Text(it.text, maxLines: 4, overflow: TextOverflow.ellipsis),
                  subtitle: Text('by ${it.authorHex} • ${DateTime.fromMillisecondsSinceEpoch(it.createdAtMs)}'),
                );
              },
            ),
          ),
        Positioned(
          right: 16, bottom: 16,
          child: FloatingActionButton.extended(
            onPressed: () async {
              final text = await showDialog<String>(
                context: context,
                builder: (ctx) => const _ComposerDialog(),
              );
              if (text != null && text.trim().isNotEmpty) {
                await _api.createPost(text);
                _load();
              }
            },
            label: const Text('Compose'),
            icon: const Icon(Icons.edit),
          ),
        )
      ],
    );
  }
}

class _ComposerDialog extends StatefulWidget { const _ComposerDialog({super.key}); @override State<_ComposerDialog> createState() => _ComposerDialogState(); }
class _ComposerDialogState extends State<_ComposerDialog> {
  final c = TextEditingController();
  @override Widget build(BuildContext context) => AlertDialog(
    title: const Text('New Post'),
    content: TextField(controller: c, maxLines: 6, decoration: const InputDecoration(hintText: "What's happening?")),
    actions: [ TextButton(onPressed: () => Navigator.pop(context), child: const Text('Cancel')),
               FilledButton(onPressed: () => Navigator.pop(context, c.text), child: const Text('Post')) ],
  );
}
