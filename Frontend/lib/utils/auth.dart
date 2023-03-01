import 'dart:convert';

import 'package:http/http.dart' as http;
import 'package:flutter_dotenv/flutter_dotenv.dart';

import '../models/user.dart';
import './web.dart' show backendUrl;

User? loggedInUser;

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
