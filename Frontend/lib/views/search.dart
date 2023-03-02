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
    return TextButton(
      onPressed: onTap,
      child: Column(
        crossAxisAlignment: CrossAxisAlignment.start,
        children: [
          const SizedBox(height: 2),
          Text(result.text, style: Theme.of(context).textTheme.titleMedium),
          const SizedBox(height: 5),
          Text(result.id.toString(), style: Theme.of(context).textTheme.bodySmall),
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

class _SearchPageState extends State<SearchPage> {
  SearchType searchType = searchTypes[0];

  List<CompletionResult> autocompleteResults = [];
  Widget? searchResults;

  final _searchQueryController = TextEditingController();
  final _focusNode = FocusNode();

  @override
  void initState() {
    super.initState();
  }

  @override
  void dispose() {
    _searchQueryController.dispose();
    super.dispose();
  }

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
    if (searchType.name == 'tags') {
      // split the search query by spaces, and replace the last one with the result
      final List<String> tags = _searchQueryController.text.split(' ');
      tags[tags.length - 1] = result.text;
      setState(() {
        _searchQueryController.text = '${tags.join(' ')} '; // trailing space to simplify adding more tags
      });
      _focusNode.requestFocus();
      _searchQueryController.selection = TextSelection.collapsed(offset: _searchQueryController.text.length);
    } else if (searchType.name == 'users') {
      context.push('/user/${result.id}');
    } else if (searchType.name == 'questions') {
      context.push('/question/${result.id}');
    }
  }

  void searchButtonTap(BuildContext context) async {
    final q = Uri.encodeQueryComponent(_searchQueryController.text);
    final path = searchType.name == 'tags' ? 'tags' : 'posts';
    context.push('/search/$path?q=$q');
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
                    controller: _searchQueryController,
                    focusNode: _focusNode,
                    onSubmitted: (value) {
                      if (searchType.name != 'users' && autocompleteResults.isNotEmpty) {
                        autoCompleteResultTap(context, autocompleteResults[0]);
                      }
                    },
                    onChanged: (value) => getAutoCompleteResults(value),
                  ),
                ),
                const SizedBox(width: 10),
                DropdownButton(
                  items: searchTypes.map((type) => DropdownMenuItem(
                    value: type,
                    child: Text(type.displayName),
                  )).toList(),
                  onChanged: (value) {
                    setState(() {
                      searchType = value!;
                      autocompleteResults = [];
                      _searchQueryController.clear();
                    });
                  },
                  value: searchType,
                ),
                const SizedBox(width: 10),
                if (searchType.name != 'users')
                  ...[
                    ElevatedButton(
                      onPressed: () {
                        searchButtonTap(context);
                      },
                      child: const Text('Search'),
                    ),
                    const SizedBox(width: 10),
                  ]
              ]
            )
          ),
          const Divider(),
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
        ]
      ),
    );
  }
}
