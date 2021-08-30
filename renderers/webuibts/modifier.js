// This works like a main function
function modify() {
	dialgoue1();	
}

// ----------
// Modifier rules

function dialgoue1() {
	let dialogue = document.querySelector('modifier.mdf-dialogue1 + *');
	if (dialogue == undefined) {
		return;
	}
	dialogue.classList+= ' dialogue-preset1';
}
