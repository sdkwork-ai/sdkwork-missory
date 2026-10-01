Map<String, dynamic>? _sdkworkAsMap(dynamic value) {
  if (value is Map<String, dynamic>) {
    return value;
  }
  if (value is Map) {
    return value.map((key, item) => MapEntry(key.toString(), item));
  }
  return null;
}

List<dynamic>? _sdkworkAsList(dynamic value) {
  return value is List ? value : null;
}

class ProblemDetail {
  final String? type;
  final String? title;
  final int? status;
  final String? detail;
  final int? code;
  final String? traceId;
  final String? instance;

  ProblemDetail({
    this.type,
    this.title,
    this.status,
    this.detail,
    this.code,
    this.traceId,
    this.instance
  });

  factory ProblemDetail.fromJson(Map<String, dynamic> json) {
    return ProblemDetail(
      type: json['type']?.toString(),
      title: json['title']?.toString(),
      status: json['status'] is int ? json['status'] : null,
      detail: json['detail']?.toString(),
      code: json['code'] is int ? json['code'] : null,
      traceId: json['traceId']?.toString(),
      instance: json['instance']?.toString()
    );
  }

  Map<String, dynamic> toJson() {
    return <String, dynamic>{
      'type': type,
      'title': title,
      'status': status,
      'detail': detail,
      'code': code,
      'traceId': traceId,
      'instance': instance,
    };
  }
}

class SdkWorkCommandData {
  final bool? accepted;
  final String? resourceId;
  final String? status;

  SdkWorkCommandData({
    this.accepted,
    this.resourceId,
    this.status
  });

  factory SdkWorkCommandData.fromJson(Map<String, dynamic> json) {
    return SdkWorkCommandData(
      accepted: json['accepted'] is bool ? json['accepted'] : null,
      resourceId: json['resourceId']?.toString(),
      status: json['status']?.toString()
    );
  }

  Map<String, dynamic> toJson() {
    return <String, dynamic>{
      'accepted': accepted,
      'resourceId': resourceId,
      'status': status,
    };
  }
}

class MissoryPersonRequestContext {
  final String? tenantId;
  final String? organizationId;
  final String? userId;

  MissoryPersonRequestContext({
    this.tenantId,
    this.organizationId,
    this.userId
  });

  factory MissoryPersonRequestContext.fromJson(Map<String, dynamic> json) {
    return MissoryPersonRequestContext(
      tenantId: json['tenantId']?.toString(),
      organizationId: json['organizationId']?.toString(),
      userId: json['userId']?.toString()
    );
  }

  Map<String, dynamic> toJson() {
    return <String, dynamic>{
      'tenantId': tenantId,
      'organizationId': organizationId,
      'userId': userId,
    };
  }
}

class MissoryMyProfile {
  final String? userId;
  final String? displayName;
  final String? nickname;
  final String? city;
  final String? occupation;
  final String? company;
  final String? education;
  final List<String>? interests;
  final List<String>? likes;
  final List<String>? dislikes;
  final String? communicationStyle;
  final String? bio;
  final String? createdAt;
  final String? updatedAt;

  MissoryMyProfile({
    this.userId,
    this.displayName,
    this.nickname,
    this.city,
    this.occupation,
    this.company,
    this.education,
    this.interests,
    this.likes,
    this.dislikes,
    this.communicationStyle,
    this.bio,
    this.createdAt,
    this.updatedAt
  });

  factory MissoryMyProfile.fromJson(Map<String, dynamic> json) {
    return MissoryMyProfile(
      userId: json['userId']?.toString(),
      displayName: json['displayName']?.toString(),
      nickname: json['nickname']?.toString(),
      city: json['city']?.toString(),
      occupation: json['occupation']?.toString(),
      company: json['company']?.toString(),
      education: json['education']?.toString(),
      interests: (() {
        final list = _sdkworkAsList(json['interests']);
        if (list == null) {
          return null;
        }
        return list
            .map((item) => item?.toString())
            .whereType<String>()
            .toList();
      })(),
      likes: (() {
        final list = _sdkworkAsList(json['likes']);
        if (list == null) {
          return null;
        }
        return list
            .map((item) => item?.toString())
            .whereType<String>()
            .toList();
      })(),
      dislikes: (() {
        final list = _sdkworkAsList(json['dislikes']);
        if (list == null) {
          return null;
        }
        return list
            .map((item) => item?.toString())
            .whereType<String>()
            .toList();
      })(),
      communicationStyle: json['communicationStyle']?.toString(),
      bio: json['bio']?.toString(),
      createdAt: json['createdAt']?.toString(),
      updatedAt: json['updatedAt']?.toString()
    );
  }

  Map<String, dynamic> toJson() {
    return <String, dynamic>{
      'userId': userId,
      'displayName': displayName,
      'nickname': nickname,
      'city': city,
      'occupation': occupation,
      'company': company,
      'education': education,
      'interests': interests?.map((item) => item).toList(),
      'likes': likes?.map((item) => item).toList(),
      'dislikes': dislikes?.map((item) => item).toList(),
      'communicationStyle': communicationStyle,
      'bio': bio,
      'createdAt': createdAt,
      'updatedAt': updatedAt,
    };
  }
}

class MissoryMyProfileUpsertRequest {
  final String? displayName;
  final String? nickname;
  final String? city;
  final String? occupation;
  final String? company;
  final String? education;
  final List<String>? interests;
  final List<String>? likes;
  final List<String>? dislikes;
  final String? communicationStyle;
  final String? bio;

  MissoryMyProfileUpsertRequest({
    this.displayName,
    this.nickname,
    this.city,
    this.occupation,
    this.company,
    this.education,
    this.interests,
    this.likes,
    this.dislikes,
    this.communicationStyle,
    this.bio
  });

  factory MissoryMyProfileUpsertRequest.fromJson(Map<String, dynamic> json) {
    return MissoryMyProfileUpsertRequest(
      displayName: json['displayName']?.toString(),
      nickname: json['nickname']?.toString(),
      city: json['city']?.toString(),
      occupation: json['occupation']?.toString(),
      company: json['company']?.toString(),
      education: json['education']?.toString(),
      interests: (() {
        final list = _sdkworkAsList(json['interests']);
        if (list == null) {
          return null;
        }
        return list
            .map((item) => item?.toString())
            .whereType<String>()
            .toList();
      })(),
      likes: (() {
        final list = _sdkworkAsList(json['likes']);
        if (list == null) {
          return null;
        }
        return list
            .map((item) => item?.toString())
            .whereType<String>()
            .toList();
      })(),
      dislikes: (() {
        final list = _sdkworkAsList(json['dislikes']);
        if (list == null) {
          return null;
        }
        return list
            .map((item) => item?.toString())
            .whereType<String>()
            .toList();
      })(),
      communicationStyle: json['communicationStyle']?.toString(),
      bio: json['bio']?.toString()
    );
  }

  Map<String, dynamic> toJson() {
    return <String, dynamic>{
      'displayName': displayName,
      'nickname': nickname,
      'city': city,
      'occupation': occupation,
      'company': company,
      'education': education,
      'interests': interests?.map((item) => item).toList(),
      'likes': likes?.map((item) => item).toList(),
      'dislikes': dislikes?.map((item) => item).toList(),
      'communicationStyle': communicationStyle,
      'bio': bio,
    };
  }
}

class MissoryPerson {
  final String? id;
  final String? displayName;
  final List<String>? aliases;
  final String? gender;
  final String? birthday;
  final String? city;
  final String? title;
  final String? company;
  final String? avatarUrl;
  final List<String>? tags;
  final List<String>? interests;
  final List<String>? preferences;
  final String? bio;
  final Map<String, String>? contactChannels;
  final String? notes;
  final String? lastContactedAt;
  final String? createdAt;
  final String? updatedAt;

  MissoryPerson({
    this.id,
    this.displayName,
    this.aliases,
    this.gender,
    this.birthday,
    this.city,
    this.title,
    this.company,
    this.avatarUrl,
    this.tags,
    this.interests,
    this.preferences,
    this.bio,
    this.contactChannels,
    this.notes,
    this.lastContactedAt,
    this.createdAt,
    this.updatedAt
  });

  factory MissoryPerson.fromJson(Map<String, dynamic> json) {
    return MissoryPerson(
      id: json['id']?.toString(),
      displayName: json['displayName']?.toString(),
      aliases: (() {
        final list = _sdkworkAsList(json['aliases']);
        if (list == null) {
          return null;
        }
        return list
            .map((item) => item?.toString())
            .whereType<String>()
            .toList();
      })(),
      gender: json['gender']?.toString(),
      birthday: json['birthday']?.toString(),
      city: json['city']?.toString(),
      title: json['title']?.toString(),
      company: json['company']?.toString(),
      avatarUrl: json['avatarUrl']?.toString(),
      tags: (() {
        final list = _sdkworkAsList(json['tags']);
        if (list == null) {
          return null;
        }
        return list
            .map((item) => item?.toString())
            .whereType<String>()
            .toList();
      })(),
      interests: (() {
        final list = _sdkworkAsList(json['interests']);
        if (list == null) {
          return null;
        }
        return list
            .map((item) => item?.toString())
            .whereType<String>()
            .toList();
      })(),
      preferences: (() {
        final list = _sdkworkAsList(json['preferences']);
        if (list == null) {
          return null;
        }
        return list
            .map((item) => item?.toString())
            .whereType<String>()
            .toList();
      })(),
      bio: json['bio']?.toString(),
      contactChannels: (() {
        final map = _sdkworkAsMap(json['contactChannels']);
        if (map == null) {
          return null;
        }
        final result = <String, String>{};
        map.forEach((key, item) {
          final deserialized = item?.toString();
          if (deserialized is String) {
            result[key] = deserialized;
          }
        });
        return result;
      })(),
      notes: json['notes']?.toString(),
      lastContactedAt: json['lastContactedAt']?.toString(),
      createdAt: json['createdAt']?.toString(),
      updatedAt: json['updatedAt']?.toString()
    );
  }

  Map<String, dynamic> toJson() {
    return <String, dynamic>{
      'id': id,
      'displayName': displayName,
      'aliases': aliases?.map((item) => item).toList(),
      'gender': gender,
      'birthday': birthday,
      'city': city,
      'title': title,
      'company': company,
      'avatarUrl': avatarUrl,
      'tags': tags?.map((item) => item).toList(),
      'interests': interests?.map((item) => item).toList(),
      'preferences': preferences?.map((item) => item).toList(),
      'bio': bio,
      'contactChannels': contactChannels?.map((key, item) => MapEntry(key, item)),
      'notes': notes,
      'lastContactedAt': lastContactedAt,
      'createdAt': createdAt,
      'updatedAt': updatedAt,
    };
  }
}

class MissoryRelationship {
  final String? id;
  final String? personId;
  final List<String>? relationshipTypes;
  final String? startedAt;
  final String? lastContactedAt;
  final String? description;
  final String? importance;
  final int? contactCycleDays;
  final String? notes;
  final String? createdAt;
  final String? updatedAt;

  MissoryRelationship({
    this.id,
    this.personId,
    this.relationshipTypes,
    this.startedAt,
    this.lastContactedAt,
    this.description,
    this.importance,
    this.contactCycleDays,
    this.notes,
    this.createdAt,
    this.updatedAt
  });

  factory MissoryRelationship.fromJson(Map<String, dynamic> json) {
    return MissoryRelationship(
      id: json['id']?.toString(),
      personId: json['personId']?.toString(),
      relationshipTypes: (() {
        final list = _sdkworkAsList(json['relationshipTypes']);
        if (list == null) {
          return null;
        }
        return list
            .map((item) => item?.toString())
            .whereType<String>()
            .toList();
      })(),
      startedAt: json['startedAt']?.toString(),
      lastContactedAt: json['lastContactedAt']?.toString(),
      description: json['description']?.toString(),
      importance: json['importance']?.toString(),
      contactCycleDays: json['contactCycleDays'] is int ? json['contactCycleDays'] : null,
      notes: json['notes']?.toString(),
      createdAt: json['createdAt']?.toString(),
      updatedAt: json['updatedAt']?.toString()
    );
  }

  Map<String, dynamic> toJson() {
    return <String, dynamic>{
      'id': id,
      'personId': personId,
      'relationshipTypes': relationshipTypes?.map((item) => item).toList(),
      'startedAt': startedAt,
      'lastContactedAt': lastContactedAt,
      'description': description,
      'importance': importance,
      'contactCycleDays': contactCycleDays,
      'notes': notes,
      'createdAt': createdAt,
      'updatedAt': updatedAt,
    };
  }
}

class MissoryRelationshipUpsertRequest {
  final List<String>? relationshipTypes;
  final String? startedAt;
  final String? lastContactedAt;
  final String? description;
  final String? importance;
  final int? contactCycleDays;
  final String? notes;

  MissoryRelationshipUpsertRequest({
    this.relationshipTypes,
    this.startedAt,
    this.lastContactedAt,
    this.description,
    this.importance,
    this.contactCycleDays,
    this.notes
  });

  factory MissoryRelationshipUpsertRequest.fromJson(Map<String, dynamic> json) {
    return MissoryRelationshipUpsertRequest(
      relationshipTypes: (() {
        final list = _sdkworkAsList(json['relationshipTypes']);
        if (list == null) {
          return null;
        }
        return list
            .map((item) => item?.toString())
            .whereType<String>()
            .toList();
      })(),
      startedAt: json['startedAt']?.toString(),
      lastContactedAt: json['lastContactedAt']?.toString(),
      description: json['description']?.toString(),
      importance: json['importance']?.toString(),
      contactCycleDays: json['contactCycleDays'] is int ? json['contactCycleDays'] : null,
      notes: json['notes']?.toString()
    );
  }

  Map<String, dynamic> toJson() {
    return <String, dynamic>{
      'relationshipTypes': relationshipTypes?.map((item) => item).toList(),
      'startedAt': startedAt,
      'lastContactedAt': lastContactedAt,
      'description': description,
      'importance': importance,
      'contactCycleDays': contactCycleDays,
      'notes': notes,
    };
  }
}

class MissoryPersonDetail {
  final MissoryPerson? person;
  final List<MissoryRelationship>? relationships;
  final List<MissoryMemory>? recentMemories;
  final List<MissoryMemory>? commitments;
  final List<MissoryStory>? stories;
  final Map<String, dynamic>? stats;

  MissoryPersonDetail({
    this.person,
    this.relationships,
    this.recentMemories,
    this.commitments,
    this.stories,
    this.stats
  });

  factory MissoryPersonDetail.fromJson(Map<String, dynamic> json) {
    return MissoryPersonDetail(
      person: (() {
        final map = _sdkworkAsMap(json['person']);
        return map == null ? null : MissoryPerson.fromJson(map);
      })(),
      relationships: (() {
        final list = _sdkworkAsList(json['relationships']);
        if (list == null) {
          return null;
        }
        return list
            .map((item) => (() {
        final map = _sdkworkAsMap(item);
        return map == null ? null : MissoryRelationship.fromJson(map);
      })())
            .whereType<MissoryRelationship>()
            .toList();
      })(),
      recentMemories: (() {
        final list = _sdkworkAsList(json['recentMemories']);
        if (list == null) {
          return null;
        }
        return list
            .map((item) => (() {
        final map = _sdkworkAsMap(item);
        return map == null ? null : MissoryMemory.fromJson(map);
      })())
            .whereType<MissoryMemory>()
            .toList();
      })(),
      commitments: (() {
        final list = _sdkworkAsList(json['commitments']);
        if (list == null) {
          return null;
        }
        return list
            .map((item) => (() {
        final map = _sdkworkAsMap(item);
        return map == null ? null : MissoryMemory.fromJson(map);
      })())
            .whereType<MissoryMemory>()
            .toList();
      })(),
      stories: (() {
        final list = _sdkworkAsList(json['stories']);
        if (list == null) {
          return null;
        }
        return list
            .map((item) => (() {
        final map = _sdkworkAsMap(item);
        return map == null ? null : MissoryStory.fromJson(map);
      })())
            .whereType<MissoryStory>()
            .toList();
      })(),
      stats: _sdkworkAsMap(json['stats'])
    );
  }

  Map<String, dynamic> toJson() {
    return <String, dynamic>{
      'person': person?.toJson(),
      'relationships': relationships?.map((item) => item.toJson()).toList(),
      'recentMemories': recentMemories?.map((item) => item.toJson()).toList(),
      'commitments': commitments?.map((item) => item.toJson()).toList(),
      'stories': stories?.map((item) => item.toJson()).toList(),
      'stats': stats,
    };
  }
}

class MissoryTimelineEntry {
  final String? occurredAt;
  final String? kind;
  final String? title;
  final String? detail;
  final String? memoryId;
  final String? storyId;

  MissoryTimelineEntry({
    this.occurredAt,
    this.kind,
    this.title,
    this.detail,
    this.memoryId,
    this.storyId
  });

  factory MissoryTimelineEntry.fromJson(Map<String, dynamic> json) {
    return MissoryTimelineEntry(
      occurredAt: json['occurredAt']?.toString(),
      kind: json['kind']?.toString(),
      title: json['title']?.toString(),
      detail: json['detail']?.toString(),
      memoryId: json['memoryId']?.toString(),
      storyId: json['storyId']?.toString()
    );
  }

  Map<String, dynamic> toJson() {
    return <String, dynamic>{
      'occurredAt': occurredAt,
      'kind': kind,
      'title': title,
      'detail': detail,
      'memoryId': memoryId,
      'storyId': storyId,
    };
  }
}

class MissoryPersonUpsertRequest {
  final String? displayName;
  final List<String>? aliases;
  final String? gender;
  final String? birthday;
  final String? city;
  final String? title;
  final String? company;
  final String? avatarUrl;
  final List<String>? tags;
  final List<String>? interests;
  final List<String>? preferences;
  final String? bio;
  final Map<String, String>? contactChannels;
  final String? notes;
  final List<String>? relationshipTypes;

  MissoryPersonUpsertRequest({
    this.displayName,
    this.aliases,
    this.gender,
    this.birthday,
    this.city,
    this.title,
    this.company,
    this.avatarUrl,
    this.tags,
    this.interests,
    this.preferences,
    this.bio,
    this.contactChannels,
    this.notes,
    this.relationshipTypes
  });

  factory MissoryPersonUpsertRequest.fromJson(Map<String, dynamic> json) {
    return MissoryPersonUpsertRequest(
      displayName: json['displayName']?.toString(),
      aliases: (() {
        final list = _sdkworkAsList(json['aliases']);
        if (list == null) {
          return null;
        }
        return list
            .map((item) => item?.toString())
            .whereType<String>()
            .toList();
      })(),
      gender: json['gender']?.toString(),
      birthday: json['birthday']?.toString(),
      city: json['city']?.toString(),
      title: json['title']?.toString(),
      company: json['company']?.toString(),
      avatarUrl: json['avatarUrl']?.toString(),
      tags: (() {
        final list = _sdkworkAsList(json['tags']);
        if (list == null) {
          return null;
        }
        return list
            .map((item) => item?.toString())
            .whereType<String>()
            .toList();
      })(),
      interests: (() {
        final list = _sdkworkAsList(json['interests']);
        if (list == null) {
          return null;
        }
        return list
            .map((item) => item?.toString())
            .whereType<String>()
            .toList();
      })(),
      preferences: (() {
        final list = _sdkworkAsList(json['preferences']);
        if (list == null) {
          return null;
        }
        return list
            .map((item) => item?.toString())
            .whereType<String>()
            .toList();
      })(),
      bio: json['bio']?.toString(),
      contactChannels: (() {
        final map = _sdkworkAsMap(json['contactChannels']);
        if (map == null) {
          return null;
        }
        final result = <String, String>{};
        map.forEach((key, item) {
          final deserialized = item?.toString();
          if (deserialized is String) {
            result[key] = deserialized;
          }
        });
        return result;
      })(),
      notes: json['notes']?.toString(),
      relationshipTypes: (() {
        final list = _sdkworkAsList(json['relationshipTypes']);
        if (list == null) {
          return null;
        }
        return list
            .map((item) => item?.toString())
            .whereType<String>()
            .toList();
      })()
    );
  }

  Map<String, dynamic> toJson() {
    return <String, dynamic>{
      'displayName': displayName,
      'aliases': aliases?.map((item) => item).toList(),
      'gender': gender,
      'birthday': birthday,
      'city': city,
      'title': title,
      'company': company,
      'avatarUrl': avatarUrl,
      'tags': tags?.map((item) => item).toList(),
      'interests': interests?.map((item) => item).toList(),
      'preferences': preferences?.map((item) => item).toList(),
      'bio': bio,
      'contactChannels': contactChannels?.map((key, item) => MapEntry(key, item)),
      'notes': notes,
      'relationshipTypes': relationshipTypes?.map((item) => item).toList(),
    };
  }
}

class MissoryMemory {
  final String? id;
  final String? personId;
  final String? storyId;
  final String? type;
  final String? title;
  final String? content;
  final String? origin;
  final String? status;
  final double? confidence;
  final String? sourceReason;
  final String? sourceKind;
  final String? sourceRef;
  final String? importance;
  final String? occurredAt;
  final String? createdAt;
  final String? updatedAt;

  MissoryMemory({
    this.id,
    this.personId,
    this.storyId,
    this.type,
    this.title,
    this.content,
    this.origin,
    this.status,
    this.confidence,
    this.sourceReason,
    this.sourceKind,
    this.sourceRef,
    this.importance,
    this.occurredAt,
    this.createdAt,
    this.updatedAt
  });

  factory MissoryMemory.fromJson(Map<String, dynamic> json) {
    return MissoryMemory(
      id: json['id']?.toString(),
      personId: json['personId']?.toString(),
      storyId: json['storyId']?.toString(),
      type: json['type']?.toString(),
      title: json['title']?.toString(),
      content: json['content']?.toString(),
      origin: json['origin']?.toString(),
      status: json['status']?.toString(),
      confidence: json['confidence'] is num ? json['confidence'].toDouble() : null,
      sourceReason: json['sourceReason']?.toString(),
      sourceKind: json['sourceKind']?.toString(),
      sourceRef: json['sourceRef']?.toString(),
      importance: json['importance']?.toString(),
      occurredAt: json['occurredAt']?.toString(),
      createdAt: json['createdAt']?.toString(),
      updatedAt: json['updatedAt']?.toString()
    );
  }

  Map<String, dynamic> toJson() {
    return <String, dynamic>{
      'id': id,
      'personId': personId,
      'storyId': storyId,
      'type': type,
      'title': title,
      'content': content,
      'origin': origin,
      'status': status,
      'confidence': confidence,
      'sourceReason': sourceReason,
      'sourceKind': sourceKind,
      'sourceRef': sourceRef,
      'importance': importance,
      'occurredAt': occurredAt,
      'createdAt': createdAt,
      'updatedAt': updatedAt,
    };
  }
}

class MissoryMemoryUpsertRequest {
  final String? personId;
  final String? storyId;
  final String? type;
  final String? title;
  final String? content;
  final String? importance;
  final String? occurredAt;
  final String? sourceKind;
  final String? sourceRef;

  MissoryMemoryUpsertRequest({
    this.personId,
    this.storyId,
    this.type,
    this.title,
    this.content,
    this.importance,
    this.occurredAt,
    this.sourceKind,
    this.sourceRef
  });

  factory MissoryMemoryUpsertRequest.fromJson(Map<String, dynamic> json) {
    return MissoryMemoryUpsertRequest(
      personId: json['personId']?.toString(),
      storyId: json['storyId']?.toString(),
      type: json['type']?.toString(),
      title: json['title']?.toString(),
      content: json['content']?.toString(),
      importance: json['importance']?.toString(),
      occurredAt: json['occurredAt']?.toString(),
      sourceKind: json['sourceKind']?.toString(),
      sourceRef: json['sourceRef']?.toString()
    );
  }

  Map<String, dynamic> toJson() {
    return <String, dynamic>{
      'personId': personId,
      'storyId': storyId,
      'type': type,
      'title': title,
      'content': content,
      'importance': importance,
      'occurredAt': occurredAt,
      'sourceKind': sourceKind,
      'sourceRef': sourceRef,
    };
  }
}

class MissoryMemoryExtractRequest {
  final String? personId;
  final String? text;

  MissoryMemoryExtractRequest({
    this.personId,
    this.text
  });

  factory MissoryMemoryExtractRequest.fromJson(Map<String, dynamic> json) {
    return MissoryMemoryExtractRequest(
      personId: json['personId']?.toString(),
      text: json['text']?.toString()
    );
  }

  Map<String, dynamic> toJson() {
    return <String, dynamic>{
      'personId': personId,
      'text': text,
    };
  }
}

class MissoryStory {
  final String? id;
  final String? title;
  final String? summary;
  final String? summaryOrigin;
  final List<String>? participantIds;
  final List<String>? memoryIds;
  final String? startedAt;
  final String? endedAt;
  final String? location;
  final String? createdAt;
  final String? updatedAt;

  MissoryStory({
    this.id,
    this.title,
    this.summary,
    this.summaryOrigin,
    this.participantIds,
    this.memoryIds,
    this.startedAt,
    this.endedAt,
    this.location,
    this.createdAt,
    this.updatedAt
  });

  factory MissoryStory.fromJson(Map<String, dynamic> json) {
    return MissoryStory(
      id: json['id']?.toString(),
      title: json['title']?.toString(),
      summary: json['summary']?.toString(),
      summaryOrigin: json['summaryOrigin']?.toString(),
      participantIds: (() {
        final list = _sdkworkAsList(json['participantIds']);
        if (list == null) {
          return null;
        }
        return list
            .map((item) => item?.toString())
            .whereType<String>()
            .toList();
      })(),
      memoryIds: (() {
        final list = _sdkworkAsList(json['memoryIds']);
        if (list == null) {
          return null;
        }
        return list
            .map((item) => item?.toString())
            .whereType<String>()
            .toList();
      })(),
      startedAt: json['startedAt']?.toString(),
      endedAt: json['endedAt']?.toString(),
      location: json['location']?.toString(),
      createdAt: json['createdAt']?.toString(),
      updatedAt: json['updatedAt']?.toString()
    );
  }

  Map<String, dynamic> toJson() {
    return <String, dynamic>{
      'id': id,
      'title': title,
      'summary': summary,
      'summaryOrigin': summaryOrigin,
      'participantIds': participantIds?.map((item) => item).toList(),
      'memoryIds': memoryIds?.map((item) => item).toList(),
      'startedAt': startedAt,
      'endedAt': endedAt,
      'location': location,
      'createdAt': createdAt,
      'updatedAt': updatedAt,
    };
  }
}

class MissoryStoryUpsertRequest {
  final String? title;
  final List<String>? participantIds;
  final List<String>? memoryIds;
  final String? startedAt;
  final String? endedAt;
  final String? location;

  MissoryStoryUpsertRequest({
    this.title,
    this.participantIds,
    this.memoryIds,
    this.startedAt,
    this.endedAt,
    this.location
  });

  factory MissoryStoryUpsertRequest.fromJson(Map<String, dynamic> json) {
    return MissoryStoryUpsertRequest(
      title: json['title']?.toString(),
      participantIds: (() {
        final list = _sdkworkAsList(json['participantIds']);
        if (list == null) {
          return null;
        }
        return list
            .map((item) => item?.toString())
            .whereType<String>()
            .toList();
      })(),
      memoryIds: (() {
        final list = _sdkworkAsList(json['memoryIds']);
        if (list == null) {
          return null;
        }
        return list
            .map((item) => item?.toString())
            .whereType<String>()
            .toList();
      })(),
      startedAt: json['startedAt']?.toString(),
      endedAt: json['endedAt']?.toString(),
      location: json['location']?.toString()
    );
  }

  Map<String, dynamic> toJson() {
    return <String, dynamic>{
      'title': title,
      'participantIds': participantIds?.map((item) => item).toList(),
      'memoryIds': memoryIds?.map((item) => item).toList(),
      'startedAt': startedAt,
      'endedAt': endedAt,
      'location': location,
    };
  }
}

class MissoryReminder {
  final String? reminderId;
  final String? type;
  final String? personId;
  final String? personName;
  final String? title;
  final String? detail;
  final String? dueAt;
  final int? daysOverdue;
  final int? daysUntil;

  MissoryReminder({
    this.reminderId,
    this.type,
    this.personId,
    this.personName,
    this.title,
    this.detail,
    this.dueAt,
    this.daysOverdue,
    this.daysUntil
  });

  factory MissoryReminder.fromJson(Map<String, dynamic> json) {
    return MissoryReminder(
      reminderId: json['reminderId']?.toString(),
      type: json['type']?.toString(),
      personId: json['personId']?.toString(),
      personName: json['personName']?.toString(),
      title: json['title']?.toString(),
      detail: json['detail']?.toString(),
      dueAt: json['dueAt']?.toString(),
      daysOverdue: json['daysOverdue'] is int ? json['daysOverdue'] : null,
      daysUntil: json['daysUntil'] is int ? json['daysUntil'] : null
    );
  }

  Map<String, dynamic> toJson() {
    return <String, dynamic>{
      'reminderId': reminderId,
      'type': type,
      'personId': personId,
      'personName': personName,
      'title': title,
      'detail': detail,
      'dueAt': dueAt,
      'daysOverdue': daysOverdue,
      'daysUntil': daysUntil,
    };
  }
}

class MissoryReminderSnoozeRequest {
  final int? days;

  MissoryReminderSnoozeRequest({
    this.days
  });

  factory MissoryReminderSnoozeRequest.fromJson(Map<String, dynamic> json) {
    return MissoryReminderSnoozeRequest(
      days: json['days'] is int ? json['days'] : null
    );
  }

  Map<String, dynamic> toJson() {
    return <String, dynamic>{
      'days': days,
    };
  }
}

class MissoryHomeDigest {
  final List<MissoryReminder>? todayReminders;
  final List<MissoryMemory>? recentMemories;
  final List<MissoryPerson>? recentPersons;

  MissoryHomeDigest({
    this.todayReminders,
    this.recentMemories,
    this.recentPersons
  });

  factory MissoryHomeDigest.fromJson(Map<String, dynamic> json) {
    return MissoryHomeDigest(
      todayReminders: (() {
        final list = _sdkworkAsList(json['todayReminders']);
        if (list == null) {
          return null;
        }
        return list
            .map((item) => (() {
        final map = _sdkworkAsMap(item);
        return map == null ? null : MissoryReminder.fromJson(map);
      })())
            .whereType<MissoryReminder>()
            .toList();
      })(),
      recentMemories: (() {
        final list = _sdkworkAsList(json['recentMemories']);
        if (list == null) {
          return null;
        }
        return list
            .map((item) => (() {
        final map = _sdkworkAsMap(item);
        return map == null ? null : MissoryMemory.fromJson(map);
      })())
            .whereType<MissoryMemory>()
            .toList();
      })(),
      recentPersons: (() {
        final list = _sdkworkAsList(json['recentPersons']);
        if (list == null) {
          return null;
        }
        return list
            .map((item) => (() {
        final map = _sdkworkAsMap(item);
        return map == null ? null : MissoryPerson.fromJson(map);
      })())
            .whereType<MissoryPerson>()
            .toList();
      })()
    );
  }

  Map<String, dynamic> toJson() {
    return <String, dynamic>{
      'todayReminders': todayReminders?.map((item) => item.toJson()).toList(),
      'recentMemories': recentMemories?.map((item) => item.toJson()).toList(),
      'recentPersons': recentPersons?.map((item) => item.toJson()).toList(),
    };
  }
}

class MissoryAssistantQueryRequest {
  final String? question;

  MissoryAssistantQueryRequest({
    this.question
  });

  factory MissoryAssistantQueryRequest.fromJson(Map<String, dynamic> json) {
    return MissoryAssistantQueryRequest(
      question: json['question']?.toString()
    );
  }

  Map<String, dynamic> toJson() {
    return <String, dynamic>{
      'question': question,
    };
  }
}

class MissoryCitation {
  final String? kind;
  final String? id;

  MissoryCitation({
    this.kind,
    this.id
  });

  factory MissoryCitation.fromJson(Map<String, dynamic> json) {
    return MissoryCitation(
      kind: json['kind']?.toString(),
      id: json['id']?.toString()
    );
  }

  Map<String, dynamic> toJson() {
    return <String, dynamic>{
      'kind': kind,
      'id': id,
    };
  }
}

class MissoryAssistantAnswer {
  final String? answer;
  final List<MissoryCitation>? citations;

  MissoryAssistantAnswer({
    this.answer,
    this.citations
  });

  factory MissoryAssistantAnswer.fromJson(Map<String, dynamic> json) {
    return MissoryAssistantAnswer(
      answer: json['answer']?.toString(),
      citations: (() {
        final list = _sdkworkAsList(json['citations']);
        if (list == null) {
          return null;
        }
        return list
            .map((item) => (() {
        final map = _sdkworkAsMap(item);
        return map == null ? null : MissoryCitation.fromJson(map);
      })())
            .whereType<MissoryCitation>()
            .toList();
      })()
    );
  }

  Map<String, dynamic> toJson() {
    return <String, dynamic>{
      'answer': answer,
      'citations': citations?.map((item) => item.toJson()).toList(),
    };
  }
}

class MissoryBriefingRequest {
  final String? personId;

  MissoryBriefingRequest({
    this.personId
  });

  factory MissoryBriefingRequest.fromJson(Map<String, dynamic> json) {
    return MissoryBriefingRequest(
      personId: json['personId']?.toString()
    );
  }

  Map<String, dynamic> toJson() {
    return <String, dynamic>{
      'personId': personId,
    };
  }
}

class MissoryBriefing {
  final String? personId;
  final String? briefing;
  final List<String>? topics;

  MissoryBriefing({
    this.personId,
    this.briefing,
    this.topics
  });

  factory MissoryBriefing.fromJson(Map<String, dynamic> json) {
    return MissoryBriefing(
      personId: json['personId']?.toString(),
      briefing: json['briefing']?.toString(),
      topics: (() {
        final list = _sdkworkAsList(json['topics']);
        if (list == null) {
          return null;
        }
        return list
            .map((item) => item?.toString())
            .whereType<String>()
            .toList();
      })()
    );
  }

  Map<String, dynamic> toJson() {
    return <String, dynamic>{
      'personId': personId,
      'briefing': briefing,
      'topics': topics?.map((item) => item).toList(),
    };
  }
}

class MissoryMessageDraftRequest {
  final String? personId;
  final String? scenario;
  final String? tone;
  final String? note;

  MissoryMessageDraftRequest({
    this.personId,
    this.scenario,
    this.tone,
    this.note
  });

  factory MissoryMessageDraftRequest.fromJson(Map<String, dynamic> json) {
    return MissoryMessageDraftRequest(
      personId: json['personId']?.toString(),
      scenario: json['scenario']?.toString(),
      tone: json['tone']?.toString(),
      note: json['note']?.toString()
    );
  }

  Map<String, dynamic> toJson() {
    return <String, dynamic>{
      'personId': personId,
      'scenario': scenario,
      'tone': tone,
      'note': note,
    };
  }
}

class MissoryMessageDraft {
  final String? personId;
  final String? draft;
  final String? tone;
  final String? disclaimer;

  MissoryMessageDraft({
    this.personId,
    this.draft,
    this.tone,
    this.disclaimer
  });

  factory MissoryMessageDraft.fromJson(Map<String, dynamic> json) {
    return MissoryMessageDraft(
      personId: json['personId']?.toString(),
      draft: json['draft']?.toString(),
      tone: json['tone']?.toString(),
      disclaimer: json['disclaimer']?.toString()
    );
  }

  Map<String, dynamic> toJson() {
    return <String, dynamic>{
      'personId': personId,
      'draft': draft,
      'tone': tone,
      'disclaimer': disclaimer,
    };
  }
}

class MissoryChatSummaryRequest {
  final String? personId;
  final String? text;

  MissoryChatSummaryRequest({
    this.personId,
    this.text
  });

  factory MissoryChatSummaryRequest.fromJson(Map<String, dynamic> json) {
    return MissoryChatSummaryRequest(
      personId: json['personId']?.toString(),
      text: json['text']?.toString()
    );
  }

  Map<String, dynamic> toJson() {
    return <String, dynamic>{
      'personId': personId,
      'text': text,
    };
  }
}

class MissoryChatSummary {
  final String? personId;
  final String? summary;
  final List<MissoryMemory>? candidateMemories;

  MissoryChatSummary({
    this.personId,
    this.summary,
    this.candidateMemories
  });

  factory MissoryChatSummary.fromJson(Map<String, dynamic> json) {
    return MissoryChatSummary(
      personId: json['personId']?.toString(),
      summary: json['summary']?.toString(),
      candidateMemories: (() {
        final list = _sdkworkAsList(json['candidateMemories']);
        if (list == null) {
          return null;
        }
        return list
            .map((item) => (() {
        final map = _sdkworkAsMap(item);
        return map == null ? null : MissoryMemory.fromJson(map);
      })())
            .whereType<MissoryMemory>()
            .toList();
      })()
    );
  }

  Map<String, dynamic> toJson() {
    return <String, dynamic>{
      'personId': personId,
      'summary': summary,
      'candidateMemories': candidateMemories?.map((item) => item.toJson()).toList(),
    };
  }
}

class PageInfo {
  final String? mode;
  final int? page;
  final int? pageSize;
  final String? totalItems;
  final int? totalPages;
  final String? nextCursor;
  final bool? hasMore;

  PageInfo({
    this.mode,
    this.page,
    this.pageSize,
    this.totalItems,
    this.totalPages,
    this.nextCursor,
    this.hasMore
  });

  factory PageInfo.fromJson(Map<String, dynamic> json) {
    return PageInfo(
      mode: json['mode']?.toString(),
      page: json['page'] is int ? json['page'] : null,
      pageSize: json['pageSize'] is int ? json['pageSize'] : null,
      totalItems: json['totalItems']?.toString(),
      totalPages: json['totalPages'] is int ? json['totalPages'] : null,
      nextCursor: json['nextCursor']?.toString(),
      hasMore: json['hasMore'] is bool ? json['hasMore'] : null
    );
  }

  Map<String, dynamic> toJson() {
    return <String, dynamic>{
      'mode': mode,
      'page': page,
      'pageSize': pageSize,
      'totalItems': totalItems,
      'totalPages': totalPages,
      'nextCursor': nextCursor,
      'hasMore': hasMore,
    };
  }
}

class SdkWorkApiResponse {
  final int? code;
  final dynamic data;
  final String? traceId;

  SdkWorkApiResponse({
    this.code,
    this.data,
    this.traceId
  });

  factory SdkWorkApiResponse.fromJson(Map<String, dynamic> json) {
    return SdkWorkApiResponse(
      code: json['code'] is int ? json['code'] : null,
      data: json['data'],
      traceId: json['traceId']?.toString()
    );
  }

  Map<String, dynamic> toJson() {
    return <String, dynamic>{
      'code': code,
      'data': data,
      'traceId': traceId,
    };
  }
}

class SdkWorkResourceData {
  final dynamic item;

  SdkWorkResourceData({
    this.item
  });

  factory SdkWorkResourceData.fromJson(Map<String, dynamic> json) {
    return SdkWorkResourceData(
      item: json['item']
    );
  }

  Map<String, dynamic> toJson() {
    return <String, dynamic>{
      'item': item,
    };
  }
}

class SdkWorkPageData {
  final List<dynamic>? items;
  final PageInfo? pageInfo;

  SdkWorkPageData({
    this.items,
    this.pageInfo
  });

  factory SdkWorkPageData.fromJson(Map<String, dynamic> json) {
    return SdkWorkPageData(
      items: (() {
        final list = _sdkworkAsList(json['items']);
        if (list == null) {
          return null;
        }
        return list
            .map((item) => item)
            .whereType<dynamic>()
            .toList();
      })(),
      pageInfo: (() {
        final map = _sdkworkAsMap(json['pageInfo']);
        return map == null ? null : PageInfo.fromJson(map);
      })()
    );
  }

  Map<String, dynamic> toJson() {
    return <String, dynamic>{
      'items': items?.map((item) => item).toList(),
      'pageInfo': pageInfo?.toJson(),
    };
  }
}

class SdkWorkResourceResponse {
  final int? code;
  final dynamic data;
  final String? traceId;

  SdkWorkResourceResponse({
    this.code,
    this.data,
    this.traceId
  });

  factory SdkWorkResourceResponse.fromJson(Map<String, dynamic> json) {
    return SdkWorkResourceResponse(
      code: json['code'] is int ? json['code'] : null,
      data: json['data'],
      traceId: json['traceId']?.toString()
    );
  }

  Map<String, dynamic> toJson() {
    return <String, dynamic>{
      'code': code,
      'data': data,
      'traceId': traceId,
    };
  }
}

class SdkWorkListResponse {
  final int? code;
  final dynamic data;
  final String? traceId;

  SdkWorkListResponse({
    this.code,
    this.data,
    this.traceId
  });

  factory SdkWorkListResponse.fromJson(Map<String, dynamic> json) {
    return SdkWorkListResponse(
      code: json['code'] is int ? json['code'] : null,
      data: json['data'],
      traceId: json['traceId']?.toString()
    );
  }

  Map<String, dynamic> toJson() {
    return <String, dynamic>{
      'code': code,
      'data': data,
      'traceId': traceId,
    };
  }
}

class SdkWorkCommandResponse {
  final int? code;
  final dynamic data;
  final String? traceId;

  SdkWorkCommandResponse({
    this.code,
    this.data,
    this.traceId
  });

  factory SdkWorkCommandResponse.fromJson(Map<String, dynamic> json) {
    return SdkWorkCommandResponse(
      code: json['code'] is int ? json['code'] : null,
      data: json['data'],
      traceId: json['traceId']?.toString()
    );
  }

  Map<String, dynamic> toJson() {
    return <String, dynamic>{
      'code': code,
      'data': data,
      'traceId': traceId,
    };
  }
}

class MissoryMyProfileResponse {
  final int? code;
  final dynamic data;
  final String? traceId;

  MissoryMyProfileResponse({
    this.code,
    this.data,
    this.traceId
  });

  factory MissoryMyProfileResponse.fromJson(Map<String, dynamic> json) {
    return MissoryMyProfileResponse(
      code: json['code'] is int ? json['code'] : null,
      data: _sdkworkAsMap(json['data']),
      traceId: json['traceId']?.toString()
    );
  }

  Map<String, dynamic> toJson() {
    return <String, dynamic>{
      'code': code,
      'data': data,
      'traceId': traceId,
    };
  }
}

class MissoryPersonResponse {
  final int? code;
  final dynamic data;
  final String? traceId;

  MissoryPersonResponse({
    this.code,
    this.data,
    this.traceId
  });

  factory MissoryPersonResponse.fromJson(Map<String, dynamic> json) {
    return MissoryPersonResponse(
      code: json['code'] is int ? json['code'] : null,
      data: _sdkworkAsMap(json['data']),
      traceId: json['traceId']?.toString()
    );
  }

  Map<String, dynamic> toJson() {
    return <String, dynamic>{
      'code': code,
      'data': data,
      'traceId': traceId,
    };
  }
}

class MissoryPersonPageResponse {
  final int? code;
  final dynamic data;
  final String? traceId;

  MissoryPersonPageResponse({
    this.code,
    this.data,
    this.traceId
  });

  factory MissoryPersonPageResponse.fromJson(Map<String, dynamic> json) {
    return MissoryPersonPageResponse(
      code: json['code'] is int ? json['code'] : null,
      data: _sdkworkAsMap(json['data']),
      traceId: json['traceId']?.toString()
    );
  }

  Map<String, dynamic> toJson() {
    return <String, dynamic>{
      'code': code,
      'data': data,
      'traceId': traceId,
    };
  }
}

class MissoryRelationshipResponse {
  final int? code;
  final dynamic data;
  final String? traceId;

  MissoryRelationshipResponse({
    this.code,
    this.data,
    this.traceId
  });

  factory MissoryRelationshipResponse.fromJson(Map<String, dynamic> json) {
    return MissoryRelationshipResponse(
      code: json['code'] is int ? json['code'] : null,
      data: _sdkworkAsMap(json['data']),
      traceId: json['traceId']?.toString()
    );
  }

  Map<String, dynamic> toJson() {
    return <String, dynamic>{
      'code': code,
      'data': data,
      'traceId': traceId,
    };
  }
}

class MissoryPersonDetailResponse {
  final int? code;
  final dynamic data;
  final String? traceId;

  MissoryPersonDetailResponse({
    this.code,
    this.data,
    this.traceId
  });

  factory MissoryPersonDetailResponse.fromJson(Map<String, dynamic> json) {
    return MissoryPersonDetailResponse(
      code: json['code'] is int ? json['code'] : null,
      data: _sdkworkAsMap(json['data']),
      traceId: json['traceId']?.toString()
    );
  }

  Map<String, dynamic> toJson() {
    return <String, dynamic>{
      'code': code,
      'data': data,
      'traceId': traceId,
    };
  }
}

class MissoryTimelineEntryPageResponse {
  final int? code;
  final dynamic data;
  final String? traceId;

  MissoryTimelineEntryPageResponse({
    this.code,
    this.data,
    this.traceId
  });

  factory MissoryTimelineEntryPageResponse.fromJson(Map<String, dynamic> json) {
    return MissoryTimelineEntryPageResponse(
      code: json['code'] is int ? json['code'] : null,
      data: _sdkworkAsMap(json['data']),
      traceId: json['traceId']?.toString()
    );
  }

  Map<String, dynamic> toJson() {
    return <String, dynamic>{
      'code': code,
      'data': data,
      'traceId': traceId,
    };
  }
}

class MissoryMemoryResponse {
  final int? code;
  final dynamic data;
  final String? traceId;

  MissoryMemoryResponse({
    this.code,
    this.data,
    this.traceId
  });

  factory MissoryMemoryResponse.fromJson(Map<String, dynamic> json) {
    return MissoryMemoryResponse(
      code: json['code'] is int ? json['code'] : null,
      data: _sdkworkAsMap(json['data']),
      traceId: json['traceId']?.toString()
    );
  }

  Map<String, dynamic> toJson() {
    return <String, dynamic>{
      'code': code,
      'data': data,
      'traceId': traceId,
    };
  }
}

class MissoryMemoryPageResponse {
  final int? code;
  final dynamic data;
  final String? traceId;

  MissoryMemoryPageResponse({
    this.code,
    this.data,
    this.traceId
  });

  factory MissoryMemoryPageResponse.fromJson(Map<String, dynamic> json) {
    return MissoryMemoryPageResponse(
      code: json['code'] is int ? json['code'] : null,
      data: _sdkworkAsMap(json['data']),
      traceId: json['traceId']?.toString()
    );
  }

  Map<String, dynamic> toJson() {
    return <String, dynamic>{
      'code': code,
      'data': data,
      'traceId': traceId,
    };
  }
}

class MissoryStoryResponse {
  final int? code;
  final dynamic data;
  final String? traceId;

  MissoryStoryResponse({
    this.code,
    this.data,
    this.traceId
  });

  factory MissoryStoryResponse.fromJson(Map<String, dynamic> json) {
    return MissoryStoryResponse(
      code: json['code'] is int ? json['code'] : null,
      data: _sdkworkAsMap(json['data']),
      traceId: json['traceId']?.toString()
    );
  }

  Map<String, dynamic> toJson() {
    return <String, dynamic>{
      'code': code,
      'data': data,
      'traceId': traceId,
    };
  }
}

class MissoryStoryPageResponse {
  final int? code;
  final dynamic data;
  final String? traceId;

  MissoryStoryPageResponse({
    this.code,
    this.data,
    this.traceId
  });

  factory MissoryStoryPageResponse.fromJson(Map<String, dynamic> json) {
    return MissoryStoryPageResponse(
      code: json['code'] is int ? json['code'] : null,
      data: _sdkworkAsMap(json['data']),
      traceId: json['traceId']?.toString()
    );
  }

  Map<String, dynamic> toJson() {
    return <String, dynamic>{
      'code': code,
      'data': data,
      'traceId': traceId,
    };
  }
}

class MissoryReminderPageResponse {
  final int? code;
  final dynamic data;
  final String? traceId;

  MissoryReminderPageResponse({
    this.code,
    this.data,
    this.traceId
  });

  factory MissoryReminderPageResponse.fromJson(Map<String, dynamic> json) {
    return MissoryReminderPageResponse(
      code: json['code'] is int ? json['code'] : null,
      data: _sdkworkAsMap(json['data']),
      traceId: json['traceId']?.toString()
    );
  }

  Map<String, dynamic> toJson() {
    return <String, dynamic>{
      'code': code,
      'data': data,
      'traceId': traceId,
    };
  }
}

class MissoryHomeDigestResponse {
  final int? code;
  final dynamic data;
  final String? traceId;

  MissoryHomeDigestResponse({
    this.code,
    this.data,
    this.traceId
  });

  factory MissoryHomeDigestResponse.fromJson(Map<String, dynamic> json) {
    return MissoryHomeDigestResponse(
      code: json['code'] is int ? json['code'] : null,
      data: _sdkworkAsMap(json['data']),
      traceId: json['traceId']?.toString()
    );
  }

  Map<String, dynamic> toJson() {
    return <String, dynamic>{
      'code': code,
      'data': data,
      'traceId': traceId,
    };
  }
}

class MissoryAssistantAnswerResponse {
  final int? code;
  final dynamic data;
  final String? traceId;

  MissoryAssistantAnswerResponse({
    this.code,
    this.data,
    this.traceId
  });

  factory MissoryAssistantAnswerResponse.fromJson(Map<String, dynamic> json) {
    return MissoryAssistantAnswerResponse(
      code: json['code'] is int ? json['code'] : null,
      data: _sdkworkAsMap(json['data']),
      traceId: json['traceId']?.toString()
    );
  }

  Map<String, dynamic> toJson() {
    return <String, dynamic>{
      'code': code,
      'data': data,
      'traceId': traceId,
    };
  }
}

class MissoryBriefingResponse {
  final int? code;
  final dynamic data;
  final String? traceId;

  MissoryBriefingResponse({
    this.code,
    this.data,
    this.traceId
  });

  factory MissoryBriefingResponse.fromJson(Map<String, dynamic> json) {
    return MissoryBriefingResponse(
      code: json['code'] is int ? json['code'] : null,
      data: _sdkworkAsMap(json['data']),
      traceId: json['traceId']?.toString()
    );
  }

  Map<String, dynamic> toJson() {
    return <String, dynamic>{
      'code': code,
      'data': data,
      'traceId': traceId,
    };
  }
}

class MissoryMessageDraftResponse {
  final int? code;
  final dynamic data;
  final String? traceId;

  MissoryMessageDraftResponse({
    this.code,
    this.data,
    this.traceId
  });

  factory MissoryMessageDraftResponse.fromJson(Map<String, dynamic> json) {
    return MissoryMessageDraftResponse(
      code: json['code'] is int ? json['code'] : null,
      data: _sdkworkAsMap(json['data']),
      traceId: json['traceId']?.toString()
    );
  }

  Map<String, dynamic> toJson() {
    return <String, dynamic>{
      'code': code,
      'data': data,
      'traceId': traceId,
    };
  }
}

class MissoryChatSummaryResponse {
  final int? code;
  final dynamic data;
  final String? traceId;

  MissoryChatSummaryResponse({
    this.code,
    this.data,
    this.traceId
  });

  factory MissoryChatSummaryResponse.fromJson(Map<String, dynamic> json) {
    return MissoryChatSummaryResponse(
      code: json['code'] is int ? json['code'] : null,
      data: _sdkworkAsMap(json['data']),
      traceId: json['traceId']?.toString()
    );
  }

  Map<String, dynamic> toJson() {
    return <String, dynamic>{
      'code': code,
      'data': data,
      'traceId': traceId,
    };
  }
}

class MemoriesConfirmResponse {
  final int? code;
  final SdkWorkCommandData? data;
  final String? traceId;

  MemoriesConfirmResponse({
    this.code,
    this.data,
    this.traceId
  });

  factory MemoriesConfirmResponse.fromJson(Map<String, dynamic> json) {
    return MemoriesConfirmResponse(
      code: json['code'] is int ? json['code'] : null,
      data: (() {
        final map = _sdkworkAsMap(json['data']);
        return map == null ? null : SdkWorkCommandData.fromJson(map);
      })(),
      traceId: json['traceId']?.toString()
    );
  }

  Map<String, dynamic> toJson() {
    return <String, dynamic>{
      'code': code,
      'data': data?.toJson(),
      'traceId': traceId,
    };
  }
}

class MemoriesRejectResponse {
  final int? code;
  final SdkWorkCommandData? data;
  final String? traceId;

  MemoriesRejectResponse({
    this.code,
    this.data,
    this.traceId
  });

  factory MemoriesRejectResponse.fromJson(Map<String, dynamic> json) {
    return MemoriesRejectResponse(
      code: json['code'] is int ? json['code'] : null,
      data: (() {
        final map = _sdkworkAsMap(json['data']);
        return map == null ? null : SdkWorkCommandData.fromJson(map);
      })(),
      traceId: json['traceId']?.toString()
    );
  }

  Map<String, dynamic> toJson() {
    return <String, dynamic>{
      'code': code,
      'data': data?.toJson(),
      'traceId': traceId,
    };
  }
}

class RemindersDismissResponse {
  final int? code;
  final SdkWorkCommandData? data;
  final String? traceId;

  RemindersDismissResponse({
    this.code,
    this.data,
    this.traceId
  });

  factory RemindersDismissResponse.fromJson(Map<String, dynamic> json) {
    return RemindersDismissResponse(
      code: json['code'] is int ? json['code'] : null,
      data: (() {
        final map = _sdkworkAsMap(json['data']);
        return map == null ? null : SdkWorkCommandData.fromJson(map);
      })(),
      traceId: json['traceId']?.toString()
    );
  }

  Map<String, dynamic> toJson() {
    return <String, dynamic>{
      'code': code,
      'data': data?.toJson(),
      'traceId': traceId,
    };
  }
}

class RemindersSnoozeResponse {
  final int? code;
  final SdkWorkCommandData? data;
  final String? traceId;

  RemindersSnoozeResponse({
    this.code,
    this.data,
    this.traceId
  });

  factory RemindersSnoozeResponse.fromJson(Map<String, dynamic> json) {
    return RemindersSnoozeResponse(
      code: json['code'] is int ? json['code'] : null,
      data: (() {
        final map = _sdkworkAsMap(json['data']);
        return map == null ? null : SdkWorkCommandData.fromJson(map);
      })(),
      traceId: json['traceId']?.toString()
    );
  }

  Map<String, dynamic> toJson() {
    return <String, dynamic>{
      'code': code,
      'data': data?.toJson(),
      'traceId': traceId,
    };
  }
}
