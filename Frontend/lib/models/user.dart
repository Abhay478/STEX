class User {
  final String id;
  final String name;

  User(this.id, this.name);

  User.fromJson(Map<String, dynamic> json)
      : id = json['id'],
        name = json['display_name'];
}
