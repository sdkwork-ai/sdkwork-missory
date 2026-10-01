import 'dart:convert';
import '../http/client.dart';
import '../models.dart';

import 'paths.dart';
import 'response_helpers.dart';


class MissoryApi {
  final HttpClient _client;

  MissoryApi(this._client);

  /// Retrieve the owner profile of the current user.
  Future<MissoryMyProfileResponse?> myProfileRetrieve() async {
    final response = await _client.get(ApiPaths.appPath('/app/v3/api/missory/my_profile'));
    return (() {
      final map = sdkworkResponseAsMap(response);
      return map == null ? null : MissoryMyProfileResponse.fromJson(map);
    })();
  }

  /// Update the owner profile of the current user.
  Future<MissoryMyProfileResponse?> myProfileUpdate(MissoryMyProfileUpsertRequest body) async {
    final payload = body.toJson();
    final response = await _client.put(ApiPaths.appPath('/app/v3/api/missory/my_profile'), body: payload, contentType: 'application/json');
    return (() {
      final map = sdkworkResponseAsMap(response);
      return map == null ? null : MissoryMyProfileResponse.fromJson(map);
    })();
  }

  /// Create a person.
  Future<MissoryPersonResponse?> personsCreate(MissoryPersonUpsertRequest body) async {
    final payload = body.toJson();
    final response = await _client.post(ApiPaths.appPath('/app/v3/api/missory/persons'), body: payload, contentType: 'application/json');
    return (() {
      final map = sdkworkResponseAsMap(response);
      return map == null ? null : MissoryPersonResponse.fromJson(map);
    })();
  }

  /// List persons with optional keyword and relationship-type filters.
  Future<MissoryPersonPageResponse?> personsList([int? page, int? pageSize, String? q, String? relationshipType]) async {
    final query = buildQueryString([
      QueryParameterSpec('page', page, 'form', true, false, null),
      QueryParameterSpec('page_size', pageSize, 'form', true, false, null),
      QueryParameterSpec('q', q, 'form', true, false, null),
      QueryParameterSpec('relationshipType', relationshipType, 'form', true, false, null)
    ]);
    final response = await _client.get(ApiPaths.appendQueryString(ApiPaths.appPath('/app/v3/api/missory/persons'), query));
    return (() {
      final map = sdkworkResponseAsMap(response);
      return map == null ? null : MissoryPersonPageResponse.fromJson(map);
    })();
  }

  /// Retrieve a person with relationships, recent memories, and stats.
  Future<MissoryPersonDetailResponse?> personsRetrieve(String personId) async {
    final response = await _client.get(ApiPaths.appPath('/app/v3/api/missory/persons/${serializePathParameter(personId, const PathParameterSpec('personId', 'simple', false))}'));
    return (() {
      final map = sdkworkResponseAsMap(response);
      return map == null ? null : MissoryPersonDetailResponse.fromJson(map);
    })();
  }

  /// Update a person.
  Future<MissoryPersonResponse?> personsUpdate(String personId, MissoryPersonUpsertRequest body) async {
    final payload = body.toJson();
    final response = await _client.put(ApiPaths.appPath('/app/v3/api/missory/persons/${serializePathParameter(personId, const PathParameterSpec('personId', 'simple', false))}'), body: payload, contentType: 'application/json');
    return (() {
      final map = sdkworkResponseAsMap(response);
      return map == null ? null : MissoryPersonResponse.fromJson(map);
    })();
  }

  /// Delete a person and its owned relationships and memory links.
  Future<void> personsDelete(String personId) async {
    await _client.delete(ApiPaths.appPath('/app/v3/api/missory/persons/${serializePathParameter(personId, const PathParameterSpec('personId', 'simple', false))}'));
  }

  /// Relationship timeline for one person.
  Future<MissoryTimelineEntryPageResponse?> personsTimelineList(String personId) async {
    final response = await _client.get(ApiPaths.appPath('/app/v3/api/missory/persons/${serializePathParameter(personId, const PathParameterSpec('personId', 'simple', false))}/timeline'));
    return (() {
      final map = sdkworkResponseAsMap(response);
      return map == null ? null : MissoryTimelineEntryPageResponse.fromJson(map);
    })();
  }

  /// Add or update the owner-to-person relationship.
  Future<MissoryRelationshipResponse?> relationshipsCreate(String personId, MissoryRelationshipUpsertRequest body) async {
    final payload = body.toJson();
    final response = await _client.post(ApiPaths.appPath('/app/v3/api/missory/persons/${serializePathParameter(personId, const PathParameterSpec('personId', 'simple', false))}/relationships'), body: payload, contentType: 'application/json');
    return (() {
      final map = sdkworkResponseAsMap(response);
      return map == null ? null : MissoryRelationshipResponse.fromJson(map);
    })();
  }

  /// Remove one relationship edge.
  Future<void> relationshipsDelete(String personId, String relationshipId) async {
    await _client.delete(ApiPaths.appPath('/app/v3/api/missory/persons/${serializePathParameter(personId, const PathParameterSpec('personId', 'simple', false))}/relationships/${serializePathParameter(relationshipId, const PathParameterSpec('relationshipId', 'simple', false))}'));
  }

  /// Record a memory manually.
  Future<MissoryMemoryResponse?> memoriesCreate(MissoryMemoryUpsertRequest body) async {
    final payload = body.toJson();
    final response = await _client.post(ApiPaths.appPath('/app/v3/api/missory/memories'), body: payload, contentType: 'application/json');
    return (() {
      final map = sdkworkResponseAsMap(response);
      return map == null ? null : MissoryMemoryResponse.fromJson(map);
    })();
  }

  /// List memories with person/type/status/origin/keyword filters.
  Future<MissoryMemoryPageResponse?> memoriesList([int? page, int? pageSize, String? q, String? personId, String? type, String? status, String? origin]) async {
    final query = buildQueryString([
      QueryParameterSpec('page', page, 'form', true, false, null),
      QueryParameterSpec('page_size', pageSize, 'form', true, false, null),
      QueryParameterSpec('q', q, 'form', true, false, null),
      QueryParameterSpec('personId', personId, 'form', true, false, null),
      QueryParameterSpec('type', type, 'form', true, false, null),
      QueryParameterSpec('status', status, 'form', true, false, null),
      QueryParameterSpec('origin', origin, 'form', true, false, null)
    ]);
    final response = await _client.get(ApiPaths.appendQueryString(ApiPaths.appPath('/app/v3/api/missory/memories'), query));
    return (() {
      final map = sdkworkResponseAsMap(response);
      return map == null ? null : MissoryMemoryPageResponse.fromJson(map);
    })();
  }

  /// Extract candidate memories from free text (AI assist, candidates only).
  Future<MissoryMemoryPageResponse?> memoriesExtract(MissoryMemoryExtractRequest body) async {
    final payload = body.toJson();
    final response = await _client.post(ApiPaths.appPath('/app/v3/api/missory/memories/extract'), body: payload, contentType: 'application/json');
    return (() {
      final map = sdkworkResponseAsMap(response);
      return map == null ? null : MissoryMemoryPageResponse.fromJson(map);
    })();
  }

  /// Retrieve one memory including source and inference metadata.
  Future<MissoryMemoryResponse?> memoriesRetrieve(String memoryId) async {
    final response = await _client.get(ApiPaths.appPath('/app/v3/api/missory/memories/${serializePathParameter(memoryId, const PathParameterSpec('memoryId', 'simple', false))}'));
    return (() {
      final map = sdkworkResponseAsMap(response);
      return map == null ? null : MissoryMemoryResponse.fromJson(map);
    })();
  }

  /// Update one memory.
  Future<MissoryMemoryResponse?> memoriesUpdate(String memoryId, MissoryMemoryUpsertRequest body) async {
    final payload = body.toJson();
    final response = await _client.put(ApiPaths.appPath('/app/v3/api/missory/memories/${serializePathParameter(memoryId, const PathParameterSpec('memoryId', 'simple', false))}'), body: payload, contentType: 'application/json');
    return (() {
      final map = sdkworkResponseAsMap(response);
      return map == null ? null : MissoryMemoryResponse.fromJson(map);
    })();
  }

  /// Delete one memory.
  Future<void> memoriesDelete(String memoryId) async {
    await _client.delete(ApiPaths.appPath('/app/v3/api/missory/memories/${serializePathParameter(memoryId, const PathParameterSpec('memoryId', 'simple', false))}'));
  }

  /// Confirm a candidate or inference memory into a user-confirmed fact.
  Future<MemoriesConfirmResponse?> memoriesConfirm(String memoryId) async {
    final response = await _client.post(ApiPaths.appPath('/app/v3/api/missory/memories/${serializePathParameter(memoryId, const PathParameterSpec('memoryId', 'simple', false))}/confirm'));
    return (() {
      final map = sdkworkResponseAsMap(response);
      return map == null ? null : MemoriesConfirmResponse.fromJson(map);
    })();
  }

  /// Reject a candidate or inference memory.
  Future<MemoriesRejectResponse?> memoriesReject(String memoryId) async {
    final response = await _client.post(ApiPaths.appPath('/app/v3/api/missory/memories/${serializePathParameter(memoryId, const PathParameterSpec('memoryId', 'simple', false))}/reject'));
    return (() {
      final map = sdkworkResponseAsMap(response);
      return map == null ? null : MemoriesRejectResponse.fromJson(map);
    })();
  }

  /// Create a story bundling participants, events, and memories.
  Future<MissoryStoryResponse?> storiesCreate(MissoryStoryUpsertRequest body) async {
    final payload = body.toJson();
    final response = await _client.post(ApiPaths.appPath('/app/v3/api/missory/stories'), body: payload, contentType: 'application/json');
    return (() {
      final map = sdkworkResponseAsMap(response);
      return map == null ? null : MissoryStoryResponse.fromJson(map);
    })();
  }

  /// List stories.
  Future<MissoryStoryPageResponse?> storiesList([int? page, int? pageSize, String? q]) async {
    final query = buildQueryString([
      QueryParameterSpec('page', page, 'form', true, false, null),
      QueryParameterSpec('page_size', pageSize, 'form', true, false, null),
      QueryParameterSpec('q', q, 'form', true, false, null)
    ]);
    final response = await _client.get(ApiPaths.appendQueryString(ApiPaths.appPath('/app/v3/api/missory/stories'), query));
    return (() {
      final map = sdkworkResponseAsMap(response);
      return map == null ? null : MissoryStoryPageResponse.fromJson(map);
    })();
  }

  /// Retrieve one story with its timeline.
  Future<MissoryStoryResponse?> storiesRetrieve(String storyId) async {
    final response = await _client.get(ApiPaths.appPath('/app/v3/api/missory/stories/${serializePathParameter(storyId, const PathParameterSpec('storyId', 'simple', false))}'));
    return (() {
      final map = sdkworkResponseAsMap(response);
      return map == null ? null : MissoryStoryResponse.fromJson(map);
    })();
  }

  /// Update one story.
  Future<MissoryStoryResponse?> storiesUpdate(String storyId, MissoryStoryUpsertRequest body) async {
    final payload = body.toJson();
    final response = await _client.put(ApiPaths.appPath('/app/v3/api/missory/stories/${serializePathParameter(storyId, const PathParameterSpec('storyId', 'simple', false))}'), body: payload, contentType: 'application/json');
    return (() {
      final map = sdkworkResponseAsMap(response);
      return map == null ? null : MissoryStoryResponse.fromJson(map);
    })();
  }

  /// Delete one story.
  Future<void> storiesDelete(String storyId) async {
    await _client.delete(ApiPaths.appPath('/app/v3/api/missory/stories/${serializePathParameter(storyId, const PathParameterSpec('storyId', 'simple', false))}'));
  }

  /// List derived reminders (long-uncontacted, birthday, commitment, events).
  Future<MissoryReminderPageResponse?> remindersList([int? page, int? pageSize, String? type]) async {
    final query = buildQueryString([
      QueryParameterSpec('page', page, 'form', true, false, null),
      QueryParameterSpec('page_size', pageSize, 'form', true, false, null),
      QueryParameterSpec('type', type, 'form', true, false, null)
    ]);
    final response = await _client.get(ApiPaths.appendQueryString(ApiPaths.appPath('/app/v3/api/missory/reminders'), query));
    return (() {
      final map = sdkworkResponseAsMap(response);
      return map == null ? null : MissoryReminderPageResponse.fromJson(map);
    })();
  }

  /// Dismiss a derived reminder.
  Future<RemindersDismissResponse?> remindersDismiss(String reminderId) async {
    final response = await _client.post(ApiPaths.appPath('/app/v3/api/missory/reminders/${serializePathParameter(reminderId, const PathParameterSpec('reminderId', 'simple', false))}/dismiss'));
    return (() {
      final map = sdkworkResponseAsMap(response);
      return map == null ? null : RemindersDismissResponse.fromJson(map);
    })();
  }

  /// Snooze a derived reminder for a number of days.
  Future<RemindersSnoozeResponse?> remindersSnooze(String reminderId, MissoryReminderSnoozeRequest body) async {
    final payload = body.toJson();
    final response = await _client.post(ApiPaths.appPath('/app/v3/api/missory/reminders/${serializePathParameter(reminderId, const PathParameterSpec('reminderId', 'simple', false))}/snooze'), body: payload, contentType: 'application/json');
    return (() {
      final map = sdkworkResponseAsMap(response);
      return map == null ? null : RemindersSnoozeResponse.fromJson(map);
    })();
  }

  /// Home digest — today relations, recent memories, recent persons.
  Future<MissoryHomeDigestResponse?> homeTodayList() async {
    final response = await _client.get(ApiPaths.appPath('/app/v3/api/missory/home/today'));
    return (() {
      final map = sdkworkResponseAsMap(response);
      return map == null ? null : MissoryHomeDigestResponse.fromJson(map);
    })();
  }
}

class PathParameterSpec {
  final String name;
  final String style;
  final bool explode;

  const PathParameterSpec(this.name, this.style, this.explode);
}

String serializePathParameter(dynamic value, PathParameterSpec spec) {
  if (value == null) return '';
  final style = spec.style.trim().isEmpty ? 'simple' : spec.style;
  if (value is Iterable) {
    return serializePathArray(spec.name, value, style, spec.explode);
  }
  if (value is Map) {
    return serializePathObject(spec.name, value, style, spec.explode);
  }
  return pathPrimitivePrefix(spec.name, style) + Uri.encodeComponent(value.toString());
}

String serializePathArray(String name, Iterable values, String style, bool explode) {
  final serialized = values.where((item) => item != null).map((item) => Uri.encodeComponent(item.toString())).toList();
  if (serialized.isEmpty) return pathPrefix(name, style);
  if (style == 'matrix') {
    if (explode) {
      return serialized.map((item) => ';$name=$item').join();
    }
    return ';$name=${serialized.join(',')}';
  }
  final separator = explode ? '.' : ',';
  return pathPrefix(name, style) + serialized.join(separator);
}

String serializePathObject(String name, Map values, String style, bool explode) {
  final entries = <String>[];
  final exploded = <String>[];
  values.forEach((key, value) {
    if (value == null) return;
    final escapedKey = Uri.encodeComponent(key.toString());
    final escapedValue = Uri.encodeComponent(value.toString());
    if (explode) {
      if (style == 'matrix') {
        exploded.add(';$escapedKey=$escapedValue');
      } else {
        exploded.add('$escapedKey=$escapedValue');
      }
    } else {
      entries.add(escapedKey);
      entries.add(escapedValue);
    }
  });
  if (style == 'matrix') {
    if (explode) return exploded.join();
    return ';$name=${entries.join(',')}';
  }
  if (explode) {
    final separator = style == 'label' ? '.' : ',';
    return pathPrefix(name, style) + exploded.join(separator);
  }
  return pathPrefix(name, style) + entries.join(',');
}

String pathPrefix(String name, String style) {
  if (style == 'label') return '.';
  if (style == 'matrix') return ';$name';
  return '';
}

String pathPrimitivePrefix(String name, String style) {
  return style == 'matrix' ? ';$name=' : pathPrefix(name, style);
}
class QueryParameterSpec {
  final String name;
  final dynamic value;
  final String style;
  final bool explode;
  final bool allowReserved;
  final String? contentType;

  const QueryParameterSpec(
    this.name,
    this.value,
    this.style,
    this.explode,
    this.allowReserved,
    this.contentType,
  );
}

String buildQueryString(List<QueryParameterSpec> parameters) {
  final pairs = <String>[];
  for (final parameter in parameters) {
    appendSerializedParameter(pairs, parameter);
  }
  return pairs.join('&');
}

void appendSerializedParameter(List<String> pairs, QueryParameterSpec parameter) {
  final value = parameter.value;
  if (value == null) return;

  final contentType = parameter.contentType;
  if (contentType != null && contentType.trim().isNotEmpty) {
    pairs.add('${urlEncode(parameter.name)}=${encodeQueryValue(jsonEncode(value), parameter.allowReserved)}');
    return;
  }

  final style = parameter.style.trim().isEmpty ? 'form' : parameter.style;
  if (style == 'deepObject' && value is Map) {
    appendDeepObjectParameter(pairs, parameter.name, value, parameter.allowReserved);
    return;
  }
  if (value is Iterable) {
    appendArrayParameter(pairs, parameter.name, value, style, parameter.explode, parameter.allowReserved);
    return;
  }
  if (value is Map) {
    appendObjectParameter(pairs, parameter.name, value, style, parameter.explode, parameter.allowReserved);
    return;
  }
  pairs.add('${urlEncode(parameter.name)}=${encodeQueryValue(value.toString(), parameter.allowReserved)}');
}

void appendArrayParameter(
  List<String> pairs,
  String name,
  Iterable values,
  String style,
  bool explode,
  bool allowReserved,
) {
  final serialized = values.where((item) => item != null).map((item) => item.toString()).toList();
  if (serialized.isEmpty) return;
  if (style == 'form' && explode) {
    for (final item in serialized) {
      pairs.add('${urlEncode(name)}=${encodeQueryValue(item, allowReserved)}');
    }
    return;
  }
  pairs.add('${urlEncode(name)}=${encodeQueryValue(serialized.join(','), allowReserved)}');
}

void appendObjectParameter(
  List<String> pairs,
  String name,
  Map values,
  String style,
  bool explode,
  bool allowReserved,
) {
  final serialized = <String>[];
  values.forEach((key, value) {
    if (value == null) return;
    if (style == 'form' && explode) {
      pairs.add('${urlEncode(key.toString())}=${encodeQueryValue(value.toString(), allowReserved)}');
      return;
    }
    serialized.add(key.toString());
    serialized.add(value.toString());
  });
  if (serialized.isNotEmpty) {
    pairs.add('${urlEncode(name)}=${encodeQueryValue(serialized.join(','), allowReserved)}');
  }
}

void appendDeepObjectParameter(List<String> pairs, String name, Map values, bool allowReserved) {
  values.forEach((key, value) {
    if (value != null) {
      pairs.add('${urlEncode('$name[$key]')}=${encodeQueryValue(value.toString(), allowReserved)}');
    }
  });
}

String encodeQueryValue(String value, bool allowReserved) {
  var encoded = urlEncode(value);
  if (!allowReserved) return encoded;
  const replacements = <String, String>{
    '%3A': ':',
    '%2F': '/',
    '%3F': '?',
    '%23': '#',
    '%5B': '[',
    '%5D': ']',
    '%40': '@',
    '%21': '!',
    '%24': r'$',
    '%26': '&',
    '%27': "'",
    '%28': '(',
    '%29': ')',
    '%2A': '*',
    '%2B': '+',
    '%2C': ',',
    '%3B': ';',
    '%3D': '=',
  };
  replacements.forEach((escaped, reserved) {
    encoded = encoded.replaceAll(escaped, reserved);
  });
  return encoded;
}

String urlEncode(String value) => Uri.encodeQueryComponent(value);
