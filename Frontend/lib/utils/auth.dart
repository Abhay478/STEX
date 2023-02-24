import 'dart:convert';

import 'package:flutter/foundation.dart';
import 'package:http/http.dart' as http;
import 'package:flutter_dotenv/flutter_dotenv.dart';

final String backendUrl = DotEnv().env['BACKEND_URL']!;

class AuthNotifier extends ChangeNotifier {
  AuthNotifier() {
    http.get(Uri.parse('$backendUrl/me')).then((response) {
      if (response.statusCode == 200) {
        loggedInUser = jsonDecode(response.body);
      }
    });
  }

  dynamic _loggedInUser;

  dynamic get loggedInUser => _loggedInUser;

  set loggedInUser(dynamic user) {
    _loggedInUser = user;
    notifyListeners();
  }

  whoami() async {
    final response = await http.get(Uri.parse('$backendUrl/me'));

    if (response.statusCode == 200) {
      loggedInUser = jsonDecode(response.body);
    }
  }

  register(String username) async {
    final response = await http.post(Uri.parse('$backendUrl/auth/register'), body: {
      'username': username,
      // default password is the same as username
    });

    return response.statusCode == 200;
  }

  login(String username, String password) async {
    final response = await http.post(Uri.parse('$backendUrl/auth/login'), body: {
      'username': username,
      'password': password,
    });

    if (response.statusCode == 200) {
      loggedInUser = jsonDecode(response.body);
    }

    return response.statusCode == 200;
  }

  logout() async {
    await http.post(Uri.parse('$backendUrl/auth/logout'));
    loggedInUser = null;
  }
}

dynamic loggedInUser;

whoami() async {
  final response = await http.get(Uri.parse('$backendUrl/me'));

  if (response.statusCode == 200) {
    loggedInUser = jsonDecode(response.body);
    return loggedInUser;
  }
  return null;
}

register(String username) async {
  final response = await http.post(Uri.parse('$backendUrl/auth/register'), body: {
    'username': username,
    // default password is the same as username
  });

  return response.statusCode == 200;
}

login(String username, String password) async {
  final response = await http.post(Uri.parse('$backendUrl/auth/login'), body: {
    'username': username,
    'password': password,
  });

  if (response.statusCode == 200) {
    loggedInUser = jsonDecode(response.body);
    return loggedInUser;
  }
  return null;
}

logout() async {
  await http.post(Uri.parse('$backendUrl/auth/logout'));
  loggedInUser = null;
}
