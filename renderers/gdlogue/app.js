const data = $include($path(cache,out.json));
let datamap = new Map(data.nodes.map(obj => [obj.id, obj]));
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

	if (type == 'Selection') { // goto and text
		let textElem = document.createElement('div');
		textElem.textContent = div.text;
		wrapper.appendChild(textElem);
	} else if (type == 'Branch') { // goto target and quali
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
	if (!nullEmpty(gotoId)) {
		showInfo(gotoId);
	}
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
	modal.querySelector("#type").textContent = 'Type : ' + target.node_type;

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

	if(!nullEmptyArr(target.branches) || !nullEmptyArr(target.selections)) {
		let divGroup = modal.querySelector("#diversion");

		// Remove exsting elements
		purgeChildren(divGroup);

		// Set title
		let title = document.createElement('h2');
		title.textContent = "Diversion";
		divGroup.appendChild(title);

		// Starting separator
		divGroup.appendChild(document.createElement('hr'));
		if (target.node_type == "Selection") {
			target.selections.forEach((div) => {
				newDiversion(target.node_type, divGroup, div);
			})
		} else if (target.node_type == "Branch") {
			target.branches.forEach((div) => {
				newDiversion(target.node_type, divGroup, div);
			})
		}
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

// out.gv should be located in current working directory
d3.select("#graph").graphviz()
	.zoom(false)
	.renderDot(`$include(out.gv)`)
	.on('end', () => {
		let nodes = document.querySelectorAll('.node');
		nodes.forEach((node) => {
			node.addEventListener('click', showNodeInfo); 
		});
	});

document.addEventListener('keydown', ( e ) => {
	switch (e.key) {
		case 'Escape':
		case 'Backspace':
			document.getElementById('modal').style.display = 'none';
			break;
		case 'Enter':
			if (document.getElementById('modal').style.display === 'none') {
				// Show start node info
				showInfo('start');
			} else {
				// Click first link,goto in modal
				document.querySelector("#modal .goto").click();
			}
			break;
		
		// Check if given number
		default:
			let parsed = parseInt(e.key);
			// Is a number
			if ( !isNaN(parsed) ) {
				parsed -= 1; // This is because 1 should mean 0 index
				let links = document.querySelectorAll("#modal .goto");
				if (parsed < links.length) {
					links[parsed].click();
				}
			}
			break;
			
	}
});
