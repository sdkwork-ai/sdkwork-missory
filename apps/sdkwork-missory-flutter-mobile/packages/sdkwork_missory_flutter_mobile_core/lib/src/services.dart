import 'package:sdkwork_missory_app_sdk/sdkwork_missory_app_sdk.dart' as sdk;

/// Typed façades over the generated client namespaces used by the app.
///
/// The generated Dart envelopes carry `dynamic data`; the services here unwrap
/// `data.item` / `data.items` into the generated model types so screens never
/// touch raw maps.
class MissoryServices {
  MissoryServices(this._client);

  final sdk.SdkworkMissoryAppClient _client;

  Future<sdk.MissoryHomeDigest?> homeToday() async {
    final response = await _client.missory.homeTodayList();
    return _digest(response?.data);
  }

  Future<List<sdk.MissoryReminder>> reminders() async {
    final page = await _remindersPage();
    return page.$1;
  }

  Future<void> dismissReminder(String reminderId) async {
    await _client.missory.remindersDismiss(reminderId);
  }

  Future<List<sdk.MissoryPerson>> people({String? keyword}) async {
    final page = await _client.missory.personsList(1, 50, keyword);
    return _personsOf(page?.data);
  }

  Future<sdk.MissoryPersonDetail?> person(String personId) async {
    final response = await _client.missory.personsRetrieve(personId);
    final item = _itemOf(response?.data);
    if (item != null) {
      return sdk.MissoryPersonDetail.fromJson(item);
    }
    return null;
  }

  Future<sdk.MissoryPerson?> createPerson(String displayName) async {
    final response = await _client.missory.personsCreate(
      sdk.MissoryPersonUpsertRequest(displayName: displayName),
    );
    final item = _itemOf(response?.data);
    if (item != null) {
      return sdk.MissoryPerson.fromJson(item);
    }
    return null;
  }

  /// Updates the editable fields of one person (PUT /persons/{id}).
  Future<sdk.MissoryPerson?> updatePerson(String personId, sdk.MissoryPersonUpsertRequest body) async {
    final response = await _client.missory.personsUpdate(personId, body);
    final item = _itemOf(response?.data);
    if (item != null) {
      return sdk.MissoryPerson.fromJson(item);
    }
    return null;
  }

  /// Deletes one person and its owned relationships and memory links.
  Future<void> deletePerson(String personId) async {
    await _client.missory.personsDelete(personId);
  }

  /// Relationship timeline for one person.
  Future<List<sdk.MissoryTimelineEntry>> personTimeline(String personId) async {
    final page = await _client.missory.personsTimelineList(personId);
    final items = (page?.data as Map<String, dynamic>?)?['items'] as List<dynamic>? ?? [];
    return [
      for (final item in items)
        if (item is Map<String, dynamic>) sdk.MissoryTimelineEntry.fromJson(item),
    ];
  }

  /// Meeting briefing for one person (AI, informational only).
  Future<sdk.MissoryBriefing?> briefing(String personId) async {
    final response = await _client.missoryAssistant.assistantBriefingsCreate(
      sdk.MissoryBriefingRequest(personId: personId),
    );
    final item = _itemOf(response?.data);
    if (item != null) {
      return sdk.MissoryBriefing.fromJson(item);
    }
    return null;
  }

  Future<List<sdk.MissoryMemory>> memories() async {
    final page = await _client.missory.memoriesList(1, 50);
    return _memoriesOf(page?.data);
  }

  Future<List<sdk.MissoryMemory>> extractCandidates(String text) async {
    final page = await _client.missory.memoriesExtract(
      sdk.MissoryMemoryExtractRequest(text: text),
    );
    return _memoriesOf(page?.data);
  }

  Future<void> confirmMemory(String memoryId) async {
    await _client.missory.memoriesConfirm(memoryId);
  }

  Future<void> rejectMemory(String memoryId) async {
    await _client.missory.memoriesReject(memoryId);
  }

  /// Records one memory manually (required: personId, type, content).
  Future<sdk.MissoryMemory?> createMemory({
    required String personId,
    required String type,
    required String content,
  }) async {
    final response = await _client.missory.memoriesCreate(
      sdk.MissoryMemoryUpsertRequest(personId: personId, type: type, content: content),
    );
    final item = _itemOf(response?.data);
    if (item != null) {
      return sdk.MissoryMemory.fromJson(item);
    }
    return null;
  }

  /// Deletes one memory.
  Future<void> deleteMemory(String memoryId) async {
    await _client.missory.memoriesDelete(memoryId);
  }

  // ---- Owner profile ----

  Future<sdk.MissoryMyProfile?> myProfile() async {
    final response = await _client.missory.myProfileRetrieve();
    final item = _itemOf(response?.data);
    if (item != null) {
      return sdk.MissoryMyProfile.fromJson(item);
    }
    return null;
  }

  Future<sdk.MissoryMyProfile?> updateMyProfile(sdk.MissoryMyProfileUpsertRequest body) async {
    final response = await _client.missory.myProfileUpdate(body);
    final item = _itemOf(response?.data);
    if (item != null) {
      return sdk.MissoryMyProfile.fromJson(item);
    }
    return null;
  }

  /// Creates the whole-account data export document (PRD §9 privacy export).
  Future<sdk.MissoryDataExport?> exportData() async {
    final response = await _client.missory
        .dataExportsCreate(sdk.MissoryDataExportCreateRequest());
    final item = _itemOf(response?.data);
    if (item != null) {
      return sdk.MissoryDataExport.fromJson(item);
    }
    return null;
  }

  // ---- Stories ----

  Future<List<sdk.MissoryStory>> stories() async {
    final page = await _client.missory.storiesList(1, 50);
    return _storiesOf(page?.data);
  }

  Future<sdk.MissoryStory?> createStory({
    required String title,
    String? location,
    String? startedAt,
    String? endedAt,
  }) async {
    final response = await _client.missory.storiesCreate(
      sdk.MissoryStoryUpsertRequest(
        title: title,
        location: _blankToNull(location),
        startedAt: _blankToNull(startedAt),
        endedAt: _blankToNull(endedAt),
      ),
    );
    final item = _itemOf(response?.data);
    if (item != null) {
      return sdk.MissoryStory.fromJson(item);
    }
    return null;
  }

  /// Creates (or regenerates) the AI summary of one story.
  Future<sdk.MissoryStory?> summarizeStory(String storyId) async {
    final response = await _client.missoryAssistant.storiesSummariesCreate(storyId);
    final item = _itemOf(response?.data);
    if (item != null) {
      return sdk.MissoryStory.fromJson(item);
    }
    return null;
  }

  // ---- Reminders ----

  /// Snoozes one reminder for [days] days.
  Future<void> snoozeReminder(String reminderId, {int days = 3}) async {
    await _client.missory.remindersSnooze(
      reminderId,
      sdk.MissoryReminderSnoozeRequest(days: days),
    );
  }

  // ---- Assistant ----

  /// Summarizes pasted chat text; extracted memories stay read-only candidates.
  Future<sdk.MissoryChatSummary?> chatSummary(String text) async {
    final response = await _client.missoryAssistant.assistantChatSummariesCreate(
      sdk.MissoryChatSummaryRequest(text: text),
    );
    final item = _itemOf(response?.data);
    if (item != null) {
      return sdk.MissoryChatSummary.fromJson(item);
    }
    return null;
  }

  Future<sdk.MissoryAssistantAnswer?> ask(String question) async {
    final response = await _client.missoryAssistant.assistantQuery(
      sdk.MissoryAssistantQueryRequest(question: question),
    );
    final data = response?.data;
    if (data is Map<String, dynamic>) {
      return sdk.MissoryAssistantAnswer.fromJson(data);
    }
    return null;
  }

  Future<sdk.MissoryMessageDraft?> draftBirthday(String personId) async {
    final response = await _client.missoryAssistant.assistantMessageDraftsCreate(
      sdk.MissoryMessageDraftRequest(personId: personId),
    );
    final data = response?.data;
    if (data is Map<String, dynamic>) {
      return sdk.MissoryMessageDraft.fromJson(data);
    }
    return null;
  }

  Future<(List<sdk.MissoryReminder>, int)> _remindersPage() async {
    final page = await _client.missory.remindersList(1, 20);
    final items = _remindersOf(page?.data);
    return (items, items.length);
  }

  List<sdk.MissoryReminder> _remindersOf(dynamic data) {
    final items = (data as Map<String, dynamic>?)?['items'] as List<dynamic>? ?? [];
    return [
      for (final item in items)
        if (item is Map<String, dynamic>) sdk.MissoryReminder.fromJson(item),
    ];
  }

  List<sdk.MissoryPerson> _personsOf(dynamic data) {
    final items = (data as Map<String, dynamic>?)?['items'] as List<dynamic>? ?? [];
    return [
      for (final item in items)
        if (item is Map<String, dynamic>) sdk.MissoryPerson.fromJson(item),
    ];
  }

  List<sdk.MissoryMemory> _memoriesOf(dynamic data) {
    final items = (data as Map<String, dynamic>?)?['items'] as List<dynamic>? ?? [];
    return [
      for (final item in items)
        if (item is Map<String, dynamic>) sdk.MissoryMemory.fromJson(item),
    ];
  }

  List<sdk.MissoryStory> _storiesOf(dynamic data) {
    final items = (data as Map<String, dynamic>?)?['items'] as List<dynamic>? ?? [];
    return [
      for (final item in items)
        if (item is Map<String, dynamic>) sdk.MissoryStory.fromJson(item),
    ];
  }

  /// Unwraps the `data.item` envelope of single-resource responses.
  Map<String, dynamic>? _itemOf(dynamic data) {
    if (data is Map<String, dynamic>) {
      final item = data['item'];
      if (item is Map<String, dynamic>) {
        return item;
      }
    }
    return null;
  }

  /// Optional text inputs: blank stays absent on the wire.
  String? _blankToNull(String? value) {
    final trimmed = value?.trim() ?? '';
    return trimmed.isEmpty ? null : trimmed;
  }

  sdk.MissoryHomeDigest _digest(dynamic data) {
    if (data is Map<String, dynamic>) {
      return sdk.MissoryHomeDigest.fromJson(data);
    }
    return sdk.MissoryHomeDigest();
  }
}
