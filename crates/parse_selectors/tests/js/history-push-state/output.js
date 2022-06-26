history.pushState({}, '', '/path/#a');
history.pushState({ 'page_id': 1, 'user_id': 5 }, '', '/path/#b');
history.pushState(state, '', '/path/#c');

history.pushState({}, "", "/path/#a");
history.pushState({ "page_id": 1, "user_id": 5 }, "", "/path/#b");
history.pushState(state, "", "/path/#c");

history.pushState(
	{},
	"",
	"/path/#a"
);
history.pushState(
	{ "page_id": 1, "user_id": 5 }, "",
	"/path/#b"
);
history
	.pushState(
		state,
		"",
		"/path/#c"
	);
