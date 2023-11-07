function getMd(md_path) {
	const which_md = md_path;
	const httpRequest = new XMLHttpRequest();
	// 设置请求方式和请求地址
	httpRequest.onreadystatechange = function () {
		if (httpRequest.readyState === XMLHttpRequest.DONE) {
			// 很好，服务器已经接收到了响应。
			if (httpRequest.status === 200) {
				// console.log(httpRequest.responseText);
				let text = httpRequest.responseText;
				text = changePath_pic(text);
				document.getElementById("content").innerHTML = marked.parse(text);
			} else {
				console.log(httpRequest.status);
			}
		}
		//   else {
		// 	// 还没准备好。
		// 	console.log(httpRequest.readyState);
		// }
	};
	const md_name = `/assets/blogs${which_md}`;
	// 发送请求
	httpRequest.open("GET", md_name, true);
	httpRequest.send();
}

function changePath_pic(text: string): string {
	const reg_pic = new RegExp("!\\[(.*)\\]\\((.*)\\)", "mg");
	const result = text.replace(reg_pic, "![$1](/assets/blog/$2)");

	const reg_get_path = new RegExp(
		"(?:[.*])(?!https://|http://)(.*/)(.*).md",
		"mg",
	);
	const result_get_path = text.replace(
		reg_get_path,
		"blog.html?markdown=$1$2.md",
	);
	return result;
}
