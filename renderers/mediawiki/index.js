"use strict;"

import axios from "axios";
import fs from "fs";

class MediaWiki {
	constructor(baseUrl, botId, botPwd) {
		this.baseUrl = baseUrl;
		this.apiUrl = baseUrl + "/api.php";
		this.botId = botId;
		this.botPwd = botPwd;
	}

	/// Get page
	getPage(title) {
		axios.get(this.apiUrl, {
			params: { 
				action: "parse", prop : "wikitext", page: title, format: "json" 
			}
		}).then(response => {
			let data = response.data;
			console.log(JSON.stringify(data.parse.wikitext["*"], null, 4))
		}).catch(error => {
			//console.log(error);
			console.log("Failed to get page");
			process.exit(1);
		})
	}

	/// This always replace page's content with given content
	async postPage(title, content) {
		console.log("Requesting a token from url...");
		let loginTokenResp = await this.getLoginTKResponse();
		let loginToken = loginTokenResp.data.query.tokens.logintoken;
		let cookie = loginTokenResp.headers["set-cookie"].join(';');

		console.log("Attempting login...");
		let loginResp = await this.postLogin(loginToken, cookie);
		let loginCookie = loginResp.headers["set-cookie"].join(';');

		console.log("Requesting a csrf token...");
		let csrfResp =  await this.getCsrfResponse(loginCookie);
		let csrfToken = csrfResp.data.query.tokens.csrftoken
		
		console.log("Sending a post request...");
		let editResponse = await this.editPage( title, content, csrfToken, loginCookie);
		if (editResponse.data.edit.result == "Success") {
			console.log("Successly posted a page \"" + title +"\" to " + this.baseUrl);
		}
	}

	/// GEt 
	async getLoginTKResponse() {
		// Get Login Token
		return axios.get(this.apiUrl, {
			params : { action: "query", meta: "tokens", type: "login", format: "json" }
		}).then(response => {
			return response;
		}).catch(error => {
			//console.log(error);
			console.log("Failed to get a login token");
			process.exit(1);
		})
	}

	async postLogin(loginToken, cookie) {
		let body = {
			action: 'login',
			lgname: this.botId,
			lgpassword: this.botPwd,
			lgtoken: loginToken,
			format: 'json'
		}
		let bodyData = new URLSearchParams(body).toString();

		// Log in 
		return axios.post(this.apiUrl, bodyData, { 
			headers: {
				Cookie: cookie,
			} 
		}).then(resp => {
			let result = resp.data.login.result;
			if (result != "Success") {
				console.log("Login failed with result : " + result);
				process.exit(1);
			}
			return resp;
		}).catch(error => {
			//console.log(error);
			console.log("Failed to login");
			process.exit(1);
		})
	}

	async getCsrfResponse(cookie) {
		// Get csrf token if login was successful
		return axios.get(this.apiUrl, {
			params: {
				action: "query",
				meta: "tokens",
				format: "json" 
			}, 
			headers: {
				Cookie: cookie
			}
		}).then(response => {
			return response;
		}).catch(error => {
			//console.log(error);
			console.log("Failed to get csrf token");
			process.exit(1);
		})
	}

	async editPage(title, content, csrfToken, cookie) {
		// Post request
		let body = {
			action: "edit",
			title: title,
			text: content,
			token: csrfToken,
			format: "json"
		}
		let bodyData = new URLSearchParams(body).toString();
		return axios.post( this.apiUrl, bodyData, { 
			headers: {
				Cookie: cookie
				// I don't know how to enable this...
				//"Content-Type": "multipart/form-data"
			} 
		}).then(response => {
			return response;
		}).catch(error => {
			//console.log(error);
			console.log("Failed to post a page");
			process.exit(1);
		})
	}

}

function main() {
	// Initial Config
	// dotenv.config();
	axios.defaults.withCredentials = true;

	const file_path = process.argv[2];
	const config_path = process.argv[3];
	const file_content = fs.readFileSync(file_path, 'utf-8');
	const config = JSON.parse(fs.readFileSync(config_path, 'utf-8'));

	let mw = new MediaWiki(config.env.url, config.env.bot_id, config.env.bot_pwd);
	mw.postPage(config.env.page_title, file_content);

	// Former codes for references
	//let mw = new MediaWiki("http://wiki.simoncreek.xyz/w", process.env.bot_id, process.env.bot_pwd);
	//mw.postPage(process.env.page_title, file_content);

}

// -----------------------
// Main function
main();
