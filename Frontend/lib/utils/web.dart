import 'dart:convert';

import 'package:http/http.dart' as http;

// TODO: switch to flutter_dotenv?
const String backendUrl = 'http://localhost:8080';
final client = http.Client();

class CompletionResult {
  late final int id;
  late final String text;

  CompletionResult(this.id, this.text);

  // fromjson
  CompletionResult.fromJson(Map<String, dynamic> json) {
    for (final key in json.keys) {
      if (key == 'id') {
        id = json[key];
      } else {
        text = json[key];
      }
    }
  }

  // list from json array
  static List<CompletionResult> listFromJson(List<dynamic> json) {
    return json.map((e) => CompletionResult.fromJson(e)).toList();
  }
}

class SearchType {
  const SearchType(this.name, this.displayName, this.placeholder, this.route);
  final String name;
  final String displayName;
  final String placeholder;
  final String route; // used for the URL
}

Future<List<CompletionResult>> getCompletionResults(String value, SearchType type) async {
  //return <CompletionResult>[
    //CompletionResult(1, 'test'),
    //CompletionResult(2, 'test2'),
  //];

  final uri = Uri.parse('$backendUrl/auto/${type.route}').replace(queryParameters: {'q': value});
  final response = await client.get(uri);
  if (response.statusCode == 200) {
    return CompletionResult.listFromJson(jsonDecode(response.body));
  } else {
    return [];
  }
}
