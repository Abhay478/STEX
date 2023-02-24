import 'package:flutter/material.dart';
import 'package:go_router/go_router.dart';
import 'package:stex_web/utils/app_bar.dart';
import 'package:stex_web/utils/web.dart';

class SearchBar extends StatefulWidget {
  const SearchBar({super.key});

  @override
  State<SearchBar> createState() => _SearchBarState();
}

class _SearchBarState extends State<SearchBar> {
  @override
  Widget build(BuildContext context) {
    return const Placeholder();
  }
}

class AutoCompleteResult extends StatelessWidget {
  const AutoCompleteResult({super.key, required this.result, required this.onTap});

  final CompletionResult result;
  final VoidCallback? onTap;

  @override
  Widget build(BuildContext context) {
    return OutlinedButton(
      onPressed: onTap,
      child: Column(
        crossAxisAlignment: CrossAxisAlignment.start,
        children: [
          Text(result.text, style: Theme.of(context).textTheme.titleLarge),
          Text(result.id.toString(), style: Theme.of(context).textTheme.titleSmall),
          const Divider(),
        ]
      )
    );
  }
}

class SearchPage extends StatefulWidget {
  const SearchPage({super.key});

  @override
  State<SearchPage> createState() => _SearchPageState();
}

const _searchTypes = [
  SearchType('tags', 'Tags', 'Enter space-separated list of tags', 't'),
  SearchType('users', 'Users', 'Search for a user', 'u'),
  SearchType('questions', 'Questions', 'Search for a question', 'p'),
];

class _SearchPageState extends State<SearchPage> {
  SearchType searchType = _searchTypes[0];

  List<CompletionResult> autocompleteResults = [];
  Widget? searchResults;

  final _controller = TextEditingController();

  void getAutoCompleteResults(String value) async {
    final String val;
    if (searchType.name == 'tags') {
      final List<String> tags = value.split(' ');
      val = tags[tags.length - 1];
    } else {
      val = value;
    }

    final List<CompletionResult> res;
    if (val.length < 3) {
      res = [];
    } else {
      res = await getCompletionResults(val, searchType);
    }
    setState(() {
      autocompleteResults = res;
    });
  }

  void autoCompleteResultTap(BuildContext context, CompletionResult result) {
    // TODO
    if (searchType.name == 'tags') {
      // split the search query by spaces, and replace the last one with the result
      final List<String> tags = _controller.text.split(' ');
      tags[tags.length - 1] = result.text;
      setState(() {
        _controller.text = tags.join(' ');
      });
    } else if (searchType.name == 'users') {
      // TODO: create user page
    } else if (searchType.name == 'questions') {
      context.pushNamed('/question/${result.id}');
    }
  }

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      appBar: buildAppBar(context),
      body: Column(
        children: [
          SizedBox(
            height: 50,
            child: Row(
              crossAxisAlignment: CrossAxisAlignment.stretch,
              children: [
                const SizedBox(width: 10),
                Expanded(
                  child: TextField(
                    decoration: InputDecoration(
                      hintText: searchType.placeholder,
                    ),
                    controller: _controller,
                    onChanged: (value) => getAutoCompleteResults(value),
                  ),
                ),
                const SizedBox(width: 10),
                DropdownButton(
                  items: _searchTypes.map((type) => DropdownMenuItem(
                    value: type,
                    child: Text(type.displayName),
                  )).toList(),
                  onChanged: (value) {
                    setState(() {
                      searchType = value!;
                      _controller.clear();
                    });
                  },
                  value: searchType,
                ),
                const SizedBox(width: 10),
                ElevatedButton(
                  onPressed: () {
                    // TODO
                  },
                  child: const Text('Search'),
                ),
                const SizedBox(width: 10),
              ]
            )
          ),
          //Stack(
            //// TODO: first child is the search results, second child is the autocomplete results
            //children: [
              Expanded(
                child: ListView(
                  children: autocompleteResults.map((result) {
                    return AutoCompleteResult(
                      result: result,
                      onTap: () => autoCompleteResultTap(context, result),
                    );
                  }).toList(),
                )
              )
            //]
          //)
        ]
      ),
    );
  }
}
