import 'package:intl/intl.dart';

final DateFormat formatLong = DateFormat("MMM d, y 'at' h:mm a");
final DateFormat formatShort = DateFormat("MMM d, y");

class Post {
  int? id;
  int? parentId;
  String? title;
  String? tags;
  String? body;
  int? score;
  DateTime? creationDate;
  String? ownerDisplayName;
  int? ownerUserId;

  Post.fromJson(Map<String, dynamic> json) {
    id = json['id'];
    parentId = json['parent_id'];
    title = json['title'];
    tags = json['tags'];
    body = json['body'];
    score = json['score'];
    creationDate = DateTime.parse(json['creation_date']);
    ownerDisplayName = json['owner_display_name'];
    ownerUserId = json['owner_user_id'];
  }

  static List<Post> listFromJson(List<dynamic> json) {
    return json.map((e) => Post.fromJson(e)).toList();
  }

  String creationDateShort() {
    return creationDate == null ? '' : formatShort.format(creationDate!);
  }

  String creationDateLong() {
    return creationDate == null ? '' : formatLong.format(creationDate!);
  }
}
