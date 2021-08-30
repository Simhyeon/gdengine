// ----------
// Callbacks
function addCallback(triggerId, eventType, callback) {
	document.getElementById(triggerId).addEventListener(eventType, callback);
}

function setProperties(id, props) {
	let elem = document.getElementById(id);
	for (key in props) {
		elem.setAttribute(key, props[key]);
	}
}

function toggleElement(id) {
	let elem = document.getElementById(id);
	// Enabled
	if (elem.getAttribute("disabled") == null) {
		console.log("Disabled");
		elem.classList.add("disabled");
		elem.disabled = true;
	} 
	// Disabled
	else {
		console.log("Enabled");
		elem.classList.remove("disabled");
		elem.disabled = false;
	}
}

function syncText(id, ev) {
	let elem = document.getElementById(id);
	elem.textContent = ev.target.value;
}

function triggerEvent(id, eventType) {
	let elem = document.getElementById(id);
	elem.dispatchEvent(new Event(eventType, { 'bubbles': true }))
}

function callModal(id) {
	let elem = document.getElementById(id);
	let modal = new bootstrap.Modal(elem, {keyboard: false});
	modal.show();
}

function hideModal(caller) {
	let elem = document.querySelector("#" + caller.id + " .modal-close")
	elem.dispatchEvent(new Event('click'), {'bubbles': true})
}

function toggleSidebar(id) {
	let elem = document.getElementById(id);
	let canvas = new bootstrap.Offcanvas(elem);
	canvas.toggle();
}

function onlyNumberKey(evt) {

	// Only ASCII character in that range allowed
	var ASCIICode = (evt.which) ? evt.which : evt.keyCode
	if (ASCIICode > 31 && (ASCIICode < 48 || ASCIICode > 57))
		return false;
	return true;
}

function toggleFloatMenu(id) {
	let elem = document.getElementById(id);
	if (elem.style.visibility === 'visible') {
		elem.style.visibility = 'hidden';
	} else {
		elem.style.visibility = 'visible';
	}
}
