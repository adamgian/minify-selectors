window.history.replaceState({}, '', '/path/#SELECTOR-1');
window.history.replaceState({ 'page_id': 1, 'user_id': 5 }, '', '/path/#SELECTOR-2');
window.history.replaceState(state, '', '/path/#SELECTOR-3');

history.replaceState({}, '', '/path/#SELECTOR-1');
history.replaceState({ 'page_id': 1, 'user_id': 5 }, '', '/path/#SELECTOR-2');
history.replaceState(state, '', '/path/#SELECTOR-3');

history.replaceState({}, "", "/path/#SELECTOR-1");
history.replaceState({ "page_id": 1, "user_id": 5 }, "", "/path/#SELECTOR-2");
history.replaceState(state, "", "/path/#SELECTOR-3");

history.replaceState(
	{},
	"",
	"/path/#SELECTOR-1"
);
history.replaceState(
	{ "page_id": 1, "user_id": 5 }, "",
	"/path/#SELECTOR-2"
);
history
	.replaceState(
		state,
		"",
		"/path/#SELECTOR-3"
	);

history.replaceState({}, '', foo);
history.replaceState(foo, '', bar);
