import Ajv from "ajv";
import fs from "fs";
import { schema } from "./schema.js";

function nullEmpty(str) {
	if (str == null || str.toLowerCase() == 'null' || str == ''){
		return true;
	} else {
		return false;
	}
}

class Gdlogue {
	constructor(json) { 
		this.ajv = new Ajv();
		this.validater = this.ajv.compile(schema)
		this.content = json;

		this.error_list = [];
		this.warning_list = [];
	}

	format_validate() {
		if (!this.validater(this.content)) {
			console.log(this.validater.errors);
			return false;
		} else {
			return true;
		}
	}

	content_validate() {
		this.content.forEach((node) => {
			switch (node.type) {
				case 'text':
					if (node.text === '') {
						this.warning_list.push('<Warning> : ' + node.id + '\'s text is empty.\n');
					}
					if (node.speaker === '') {
						this.warning_list.push('<Warning> : ' + node.id + '\'s speaker is empty.\n');
					}
					break;
				case 'selection':
					if (node.diversion.length == 0 ) {
						this.error_list.push('<Error> : ' + node.id + '\'s diversion array is empty. This node\' type is selection thus this is not allowed.\n');
					}
					node.diversion.forEach((div) => {
						if (div.text === '') {
							this.warning_list.push('<Warning> : ' + node.id + '\'s diversion, ' + div.id + '\' has empty text although node type is selection.\n');
						}
					});
					break;
				case 'branch':
					if (node.diversion.length == 0 ) {
						this.error_list.push('<Error> : ' + node.id + '\'s diversion array is empty. This node\' type is branch thus this is not allowed.\n');
					}
					let empty_qual_count = 0;
					node.diversion.forEach((div) => {
						if (div.qual === '') { empty_qual_count++; }
						// To only print once.
						// There is no break statement in js foreach
						if (empty_qual_count == 1){
							this.warning_list.push('<Warning> : ' + node.id + '\'s diversion include multiple empty qualifications although node type is branch. Multiple empty(default) node might not work as intended.\n');
							return;
						}
					});
					break;
				
				// Null is fine
				case null:
					break;
				case 'null':
					break;

				default:
					this.error_list.push(`<Error> : ${node.id}'s type ${node.type} is not valid\n`);
					break;
			}
		});
	}

	print() {
		console.log(JSON.stringify(this.content));
	}

	pretty_print(){
		console.log(JSON.stringify(this.content, null, 4));
	}

	dotify(){
		// Start
		let dotScript = 'digraph Dialogue {\n';
		let globalAttributes = 'node [shape="record"]\n';

		dotScript += globalAttributes;
		this.content.forEach((node) => {
			// Continue if type is null (Null type means invalid or placeholder)
			if (nullEmpty(node.type)) {
				return
			}

			// Set node attributes
			let label = `{${node.id}|${node.type}}|`;
			let style ='';
			switch (node.type) {
				case 'text':
					label += `${node.speaker}|`;
					break;
				case 'selection':
					label += `${node.speaker}|`;
					style += `colorfill="white" color="green3"`;
					break;
				case 'branch':
					style += `colorfill="white" color="dodgerblue3"`;
					break;
				default:
					break;
					
			}

			// Set node edges TODO : Check if this is valid syntax
			let edges = '';

			if (node.type == 'text' && !nullEmpty(node.goto)) {
				edges += `${node.id} -> ${node.goto}\n`;
			} else if (node.type == 'selection' || node.type == 'branch') {
				node.diversion.forEach((div) => {
					if (!nullEmpty(div.goto)){
						edges += `${node.id} -> ${div.goto}\n`;
					}
				});
			}
			dotScript += `${node.id} [id="${node.id}" label="{${label.slice(0,-1)}}" ${style}]\n`;
			dotScript += edges;
		});

		// End
		dotScript += '}';
		fs.writeFileSync('out.gv', dotScript);
	}
}

function main() {
	const file_path = process.argv[2];
	const sub_command = process.argv[3];
	if (sub_command == "" || sub_command == null) {
		console.log("Aborting because no subcommand was given.")
		return;
	}
	let json = JSON.parse(fs.readFileSync(file_path));
	let gdlogue = new Gdlogue(json);

	if (!gdlogue.format_validate()) {
		console.log("Failed to validate json file.");
		return;
	}

	gdlogue.content_validate();

	if (gdlogue.warning_list.length != 0) { console.log(...gdlogue.warning_list);}
	if (gdlogue.error_list.length != 0 ) {console.log(...gdlogue.error_list);}

	if (gdlogue.error_list.length != 0 ) {
		process.exit(0);
	}

	switch (sub_command) {
		case 'print':
			gdlogue.pretty_print();
			break;
		case 'data':
			gdlogue.print();
			break;
		case 'dotify':
			gdlogue.dotify();
			break;
		
		default:
			console.log(`"${sub_command}" is not viable sub command`);
			console.log("Use either <print> or <data>")
			return;
	}
}

main();
