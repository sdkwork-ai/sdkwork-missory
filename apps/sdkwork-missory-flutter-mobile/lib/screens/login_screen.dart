import 'package:flutter/material.dart';

import 'package:sdkwork_missory_flutter_mobile_core/sdkwork_missory_flutter_mobile_core.dart';

/// Credential-entry login screen (IAM app-api password grant). On success the
/// dual-token pair lands in the shared preferences store and the generated
/// client dispatches Authorization/Access-Token on every request.
class LoginScreen extends StatefulWidget {
  const LoginScreen({super.key, required this.runtime, required this.onSessionEstablished});

  final MissoryRuntime runtime;
  final VoidCallback onSessionEstablished;

  @override
  State<LoginScreen> createState() => _LoginScreenState();
}

class _LoginScreenState extends State<LoginScreen> {
  final _account = TextEditingController();
  final _password = TextEditingController();
  String? _error;
  bool _pending = false;

  @override
  void dispose() {
    _account.dispose();
    _password.dispose();
    super.dispose();
  }

  Future<void> _submit() async {
    if (_pending) return;
    setState(() {
      _pending = true;
      _error = null;
    });
    try {
      await widget.runtime.session.loginWithPassword(
        account: _account.text,
        password: _password.text,
      );
      widget.onSessionEstablished();
    } on MissoryLoginException catch (error) {
      setState(() {
        _error = error.message;
      });
    } catch (error) {
      setState(() {
        _error = '登录失败：$error';
      });
    } finally {
      if (mounted) {
        setState(() {
          _pending = false;
        });
      }
    }
  }

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      appBar: AppBar(title: const Text('念忆 · Missory 登录')),
      body: Center(
        child: ConstrainedBox(
          constraints: const BoxConstraints(maxWidth: 360),
          child: Padding(
            padding: const EdgeInsets.all(24),
            child: Column(
              mainAxisSize: MainAxisSize.min,
              crossAxisAlignment: CrossAxisAlignment.stretch,
              children: [
                TextField(
                  controller: _account,
                  decoration: const InputDecoration(
                    labelText: '账号（用户名 / 邮箱 / 手机号）',
                    border: OutlineInputBorder(),
                  ),
                  autocorrect: false,
                  enableSuggestions: false,
                ),
                const SizedBox(height: 12),
                TextField(
                  controller: _password,
                  obscureText: true,
                  decoration: const InputDecoration(
                    labelText: '密码',
                    border: OutlineInputBorder(),
                  ),
                ),
                if (_error != null) ...[
                  const SizedBox(height: 12),
                  Text(
                    _error!,
                    style: TextStyle(color: Theme.of(context).colorScheme.error),
                  ),
                ],
                const SizedBox(height: 20),
                FilledButton(
                  onPressed: _pending ? null : _submit,
                  child: Text(_pending ? '登录中…' : '登录'),
                ),
                const SizedBox(height: 12),
                Text(
                  '${widget.runtime.environment.profileId} · ${widget.runtime.environment.environment}',
                  textAlign: TextAlign.center,
                  style: Theme.of(context).textTheme.bodySmall,
                ),
              ],
            ),
          ),
        ),
      ),
    );
  }
}
