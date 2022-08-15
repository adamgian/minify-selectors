history.pushState({}, '', '/path/#SELECTOR-1');
history.pushState({ 'page_id': 1, 'user_id': 5 }, '', '/path/#SELECTOR-2');
history.pushState(state, '', '/path/#SELECTOR-3');

history.pushState({}, "", "/path/#SELECTOR-1");
history.pushState({ "page_id": 1, "user_id": 5 }, "", "/path/#SELECTOR-2");
history.pushState(state, "", "/path/#SELECTOR-3");

history.pushState(
	{},
	"",
	"/path/#SELECTOR-1"
);
history.pushState(
	{ "page_id": 1, "user_id": 5 }, "",
	"/path/#SELECTOR-2"
);
history
	.pushState(
		state,
		"",
		"/path/#SELECTOR-3"
	);

history.pushState({}, '');
history.pushState({'page_id': 1, 'user_id': 5}, '');
