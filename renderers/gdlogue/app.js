const data = include(out.json);
let datamap = new Map(data.map(obj => [obj.id, obj]));
let modal = document.querySelector("#modal");

function purgeChildren(node) {
	while(node.firstChild) {
		node.removeChild(node.firstChild);
	}
}

function nullEmpty(str) {
	if (str == undefined || str == null || str.toLowerCase() == 'null' || str == ''){
		return true;
	} else {
		return false;
	}
}
function nullEmptyArr(arr) {
	if (arr == undefined || arr == null || arr.length == 0){
		return true;
	} else {
		return false;
	}
}

function newDiversion(type,parentNode, div) {
	// if goto is empty, it is placeholder
	if (nullEmpty( div.goto )) { return; }
	let wrapper = document.createElement('div');
	wrapper.classList = 'diversion';

	if (type == 'selection') { // goto and text
		let textElem = document.createElement('div');
		textElem.textContent = div.text;
		wrapper.appendChild(textElem);
	} else if (type == 'branch') { // goto target and quali
		let targetElem = document.createElement('div');
		targetElem.textContent = 'Target : ' + div.target;
		wrapper.appendChild(targetElem);

		let qualElem = document.createElement('div');
		qualElem.textContent = 'Qualification : ' + div.qual;
		wrapper.appendChild(qualElem);
	}

	let gotoElem = document.createElement('div');
	gotoElem.textContent = 'Goto -> ' + div.goto;
	gotoElem.classList = "goto";
	gotoElem.dataset.goto = div.goto;
	gotoElem.addEventListener('click', showNextNodeInfo);
	wrapper.appendChild(gotoElem);

	parentNode.appendChild(wrapper);
}

function showNextNodeInfo(event) {
	let gotoId = event.currentTarget.dataset.goto;
	showInfo(gotoId);
}

function showNodeInfo(event) {
	let id = event.currentTarget.id;
	showInfo(id);
}

function showInfo(id) {
	modal.style.display = "flex";
	let target = datamap.get(id);

	// This fields are mandatory
	modal.querySelector("#id").textContent = id;
	modal.querySelector("#type").textContent = 'Type : ' + target.type;

	// These field are optional
	// Set only when not empty
	let speakerText = 'Speaker : ';
	if (!nullEmpty(target.speaker)){ speakerText += target.speaker;}
	else {speakerText = '';}
	modal.querySelector("#speaker").textContent = speakerText;

	let texty = 'Text : ';
	if (!nullEmpty(target.text)){ texty += target.text;}
	else {texty = '';}
	modal.querySelector("#texty").textContent = texty;

	let gotoText = 'Goto -> ';
	if (!nullEmpty(target.goto)){ gotoText += target.goto; }
	else {gotoText = '';}
	let gotoElem = modal.querySelector("#goto");
	gotoElem.textContent = gotoText
	gotoElem.classList = "goto";
	gotoElem.dataset.goto = target.goto;

	if(!nullEmptyArr(target.diversion)) {
		let divGroup = modal.querySelector("#diversion");

		// Remove exsting elements
		purgeChildren(divGroup);

		// Set title
		let title = document.createElement('h2');
		title.textContent = "Diversion";
		divGroup.appendChild(title);

		// Starting separator
		divGroup.appendChild(document.createElement('hr'));
		// DEBUG
		target.diversion.forEach((div) => {
			newDiversion(target.type, divGroup, div);
		})
		// Ending separator
		divGroup.appendChild(document.createElement('hr'));
	} else {
		let divGroup = modal.querySelector("#diversion");
		purgeChildren(divGroup);
	}
}

// Add close listener for close button
document.querySelector('.close-area').addEventListener('click', () => {
	document.querySelector('#modal').style.display = 'none';
});

// This should be included by m4 because it uses special syntax for gdmarp
d3.select("#graph").graphviz()
	.zoom(false)
	.renderDot(\~include(out.gv)\~)
	.on('end', () => {
		let nodes = document.querySelectorAll('.node');
		nodes.forEach((node) => {
			node.addEventListener('click', showNodeInfo); 
		});
	});

document.addEventListener('keydown', ( e ) => {
    if ( e.key === 'Escape' || e.key === 'Backspace') { // ESC
		document.getElementById('modal').style.display = 'none';
    }
});
