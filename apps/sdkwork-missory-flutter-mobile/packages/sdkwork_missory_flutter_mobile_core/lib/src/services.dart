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
    final data = response?.data;
    if (data is Map<String, dynamic>) {
      return sdk.MissoryPersonDetail.fromJson(data);
    }
    return null;
  }

  Future<sdk.MissoryPerson?> createPerson(String displayName) async {
    final response = await _client.missory.personsCreate(
      sdk.MissoryPersonUpsertRequest(displayName: displayName),
    );
    final data = response?.data;
    if (data is Map<String, dynamic>) {
      return sdk.MissoryPerson.fromJson(data);
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

  sdk.MissoryHomeDigest _digest(dynamic data) {
    if (data is Map<String, dynamic>) {
      return sdk.MissoryHomeDigest.fromJson(data);
    }
    return sdk.MissoryHomeDigest();
  }
}
