window.history.replaceState({}, '', '/path/#a');
window.history.replaceState({ 'page_id': 1, 'user_id': 5 }, '', '/path/#b');
window.history.replaceState(state, '', '/path/#c');

history.replaceState({}, '', '/path/#a');
history.replaceState({ 'page_id': 1, 'user_id': 5 }, '', '/path/#b');
history.replaceState(state, '', '/path/#c');

history.replaceState({}, "", "/path/#a");
history.replaceState({ "page_id": 1, "user_id": 5 }, "", "/path/#b");
history.replaceState(state, "", "/path/#c");

history.replaceState(
	{},
	"",
	"/path/#a"
);
history.replaceState(
	{ "page_id": 1, "user_id": 5 }, "",
	"/path/#b"
);
history
	.replaceState(
		state,
		"",
		"/path/#c"
	);

history.replaceState({}, '', foo);
history.replaceState(foo, '', bar);
