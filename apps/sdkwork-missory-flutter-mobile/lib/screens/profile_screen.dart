import 'dart:convert';

import 'package:flutter/material.dart';

import 'package:sdkwork_missory_app_sdk/sdkwork_missory_app_sdk.dart' as sdk;
import 'package:sdkwork_missory_flutter_mobile_core/sdkwork_missory_flutter_mobile_core.dart';

class ProfileScreen extends StatefulWidget {
  const ProfileScreen({super.key, required this.runtime});

  final MissoryRuntime runtime;

  @override
  State<ProfileScreen> createState() => _ProfileScreenState();
}

class _ProfileScreenState extends State<ProfileScreen> {
  final TextEditingController _displayName = TextEditingController();
  final TextEditingController _nickname = TextEditingController();
  final TextEditingController _city = TextEditingController();
  final TextEditingController _occupation = TextEditingController();
  final TextEditingController _company = TextEditingController();
  final TextEditingController _education = TextEditingController();
  final TextEditingController _interests = TextEditingController();
  final TextEditingController _likes = TextEditingController();
  final TextEditingController _dislikes = TextEditingController();
  final TextEditingController _communicationStyle = TextEditingController();
  final TextEditingController _bio = TextEditingController();
  bool _loading = true;
  bool _saving = false;
  String? _error;

  @override
  void initState() {
    super.initState();
    _refresh();
  }

  @override
  void dispose() {
    _displayName.dispose();
    _nickname.dispose();
    _city.dispose();
    _occupation.dispose();
    _company.dispose();
    _education.dispose();
    _interests.dispose();
    _likes.dispose();
    _dislikes.dispose();
    _communicationStyle.dispose();
    _bio.dispose();
    super.dispose();
  }

  Future<void> _refresh() async {
    try {
      final profile = await widget.runtime.services.myProfile();
      _fill(profile);
    } catch (error) {
      if (!mounted) return;
      setState(() {
        _loading = false;
        _error = '$error';
      });
      return;
    }
    if (!mounted) return;
    setState(() => _loading = false);
  }

  void _fill(sdk.MissoryMyProfile? profile) {
    _displayName.text = profile?.displayName ?? '';
    _nickname.text = profile?.nickname ?? '';
    _city.text = profile?.city ?? '';
    _occupation.text = profile?.occupation ?? '';
    _company.text = profile?.company ?? '';
    _education.text = profile?.education ?? '';
    _interests.text = (profile?.interests ?? const <String>[]).join(', ');
    _likes.text = (profile?.likes ?? const <String>[]).join(', ');
    _dislikes.text = (profile?.dislikes ?? const <String>[]).join(', ');
    _communicationStyle.text = profile?.communicationStyle ?? '';
    _bio.text = profile?.bio ?? '';
  }

  List<String> _splitTags(String value) {
    return [
      for (final part in value.split(','))
        if (part.trim().isNotEmpty) part.trim(),
    ];
  }

  Future<void> _save() async {
    if (_saving) return;
    setState(() => _saving = true);
    try {
      await widget.runtime.services.updateMyProfile(
        sdk.MissoryMyProfileUpsertRequest(
          displayName: _displayName.text.trim(),
          nickname: _nickname.text.trim(),
          city: _city.text.trim(),
          occupation: _occupation.text.trim(),
          company: _company.text.trim(),
          education: _education.text.trim(),
          interests: _splitTags(_interests.text),
          likes: _splitTags(_likes.text),
          dislikes: _splitTags(_dislikes.text),
          communicationStyle: _communicationStyle.text.trim(),
          bio: _bio.text.trim(),
        ),
      );
      if (!mounted) return;
      ScaffoldMessenger.of(context).showSnackBar(const SnackBar(content: Text('资料已保存')));
    } finally {
      if (mounted) setState(() => _saving = false);
    }
  }

  Future<void> _exportData() async {
    if (_saving) return;
    setState(() => _saving = true);
    try {
      final export = await widget.runtime.services.exportData();
      if (!mounted) return;
      final json = const JsonEncoder.withIndent('  ').convert(export ?? <String, dynamic>{});
      await showDialog<void>(
        context: context,
        builder: (dialogContext) => AlertDialog(
          title: const Text('我的数据导出'),
          content: SizedBox(
            width: double.maxFinite,
            child: SingleChildScrollView(
              child: SelectableText(json, style: const TextStyle(fontSize: 12)),
            ),
          ),
          actions: [
            TextButton(
              onPressed: () => Navigator.of(dialogContext).pop(),
              child: const Text('关闭'),
            ),
          ],
        ),
      );
    } catch (error) {
      if (!mounted) return;
      ScaffoldMessenger.of(context).showSnackBar(SnackBar(content: Text('导出失败：$error')));
    } finally {
      if (mounted) setState(() => _saving = false);
    }
  }

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      appBar: AppBar(title: const Text('我的资料')),
      body: _loading
          ? const Center(child: CircularProgressIndicator())
          : _error != null
              ? Center(child: Text('加载失败：$_error'))
              : SingleChildScrollView(
                  padding: const EdgeInsets.all(12),
                  child: Column(
                    children: [
                      TextField(
                        controller: _displayName,
                        decoration: const InputDecoration(labelText: '姓名', hintText: '怎么称呼你自己'),
                      ),
                      TextField(
                        controller: _nickname,
                        decoration: const InputDecoration(labelText: '昵称'),
                      ),
                      TextField(
                        controller: _city,
                        decoration: const InputDecoration(labelText: '城市'),
                      ),
                      TextField(
                        controller: _occupation,
                        decoration: const InputDecoration(labelText: '职业'),
                      ),
                      TextField(
                        controller: _company,
                        decoration: const InputDecoration(labelText: '公司'),
                      ),
                      TextField(
                        controller: _education,
                        decoration: const InputDecoration(labelText: '教育经历'),
                      ),
                      TextField(
                        controller: _interests,
                        decoration: const InputDecoration(
                          labelText: '兴趣',
                          hintText: '多个用英文逗号分隔，如：跑步, 摄影',
                        ),
                      ),
                      TextField(
                        controller: _likes,
                        decoration: const InputDecoration(labelText: '喜欢', hintText: '多个用英文逗号分隔'),
                      ),
                      TextField(
                        controller: _dislikes,
                        decoration: const InputDecoration(labelText: '不喜欢', hintText: '多个用英文逗号分隔'),
                      ),
                      TextField(
                        controller: _communicationStyle,
                        decoration: const InputDecoration(labelText: '沟通风格', hintText: '如：直接、简洁'),
                      ),
                      TextField(
                        controller: _bio,
                        decoration: const InputDecoration(labelText: '个人简介'),
                        maxLines: 3,
                      ),
                      const SizedBox(height: 16),
                      OutlinedButton(
                        onPressed: _saving ? null : _exportData,
                        child: const Text('导出我的数据（JSON）'),
                      ),
                      const SizedBox(height: 8),
                      FilledButton(
                        onPressed: _saving ? null : _save,
                        child: const Text('保存'),
                      ),
                    ],
                  ),
                ),
    );
  }
}
