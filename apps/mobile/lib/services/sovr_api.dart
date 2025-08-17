// Thin wrapper over flutter_rust_bridge-generated bindings.
// Replace the placeholders once FRB generates code from bridges/flutter/bridge.rs.

import 'dart:typed_data';

class SovrApi {
  SovrApi._();
  static final SovrApi instance = SovrApi._();

  Future<String> version() async {
    // Call into FRB binding: apiVersion()
    // return await api.apiVersion();
    return '0.2.0-dev';
  }

  Future<List<PostView>> fetchTimeline() async {
    // final items = await api.apiPostFetchTimeline(null, FeedFilter.home());
    // return items.map(PostView.fromBinding).toList();
    return const [];
  }

  Future<void> createPost(String text) async {
    // await api.apiPostCreate(Post(...));
  }
}

class PostView {
  final String id;
  final String text;
  final String authorHex;
  final int createdAtMs;
  const PostView({required this.id, required this.text, required this.authorHex, required this.createdAtMs});
}
