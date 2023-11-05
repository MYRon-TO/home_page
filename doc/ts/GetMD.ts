import {marked} from "../node_modules/marked/marked.min.js";

const which_md: string = "test.md";
// 新建一个http请求
const httpRequest = new XMLHttpRequest();
// 设置请求方式和请求地址
httpRequest.onreadystatechange = function () {
	if (httpRequest.readyState === XMLHttpRequest.DONE) {
		// 很好，服务器已经接收到了响应。
		if (httpRequest.status === 200) {
			document.getElementById("content").innerHTML =
        marked.parse(httpRequest.responseText);
		} else {
			console.log(httpRequest.status);
		}
	} else {
		// 还没准备好。
		console.log(httpRequest.readyState);
	}
};

const md_name =`/assets/blogs/${which_md}`;
// 发送请求
httpRequest.open("GET", md_name, true);
httpRequest.send();
