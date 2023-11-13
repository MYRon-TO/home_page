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
                document.getElementById("blog_path-this").innerHTML = `${which_md}`;
            }
            else {
                console.log(httpRequest.status);
            }
        }
        //   else {
        // 	// 还没准备好。
        // 	console.log(httpRequest.readyState);
        // }
    };
    const md_name = `/yuru/assets/blogs/${which_md}.md`;
    console.log(md_name);
    // 发送请求
    httpRequest.open("GET", md_name, true);
    httpRequest.send();
}
function changePath_pic(text) {
    let result = text;
    const reg_pic = new RegExp("!\\[(.*)\\]\\((?!https://\|http://)(.*)\\)", "mg");
    const reg_get_path = new RegExp("\\[(.*)\\]\\((?!https://|http://)(?:.*)/(.*).md\\)", "mg");
    result = result.replace(reg_pic, "![$1](/yuru/assets/blogs/$2)");
    result = result.replace(reg_get_path, "[$1](/yuru/doc/blog.html?markdown=$2)");
    return result;
}
