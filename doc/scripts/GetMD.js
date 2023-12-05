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
        text = changePath_pic(text, `${which_md}`);
        document.getElementById("content").innerHTML = marked.parse(text);
        document.getElementById("blog_path-this").innerHTML = `${which_md}`;
      } else {
        const NOT_FOUND = `
# 这里什么都没有

你要找的页面不存在，可能已经被删除了。
        `;
        document.getElementById("content").innerHTML = marked.parse(NOT_FOUND);
        document.getElementById("blog_path-this").innerHTML = httpRequest.status;
      }
    }
    //   else {
    // 	// 还没准备好。
    // 	console.log(httpRequest.readyState);
    // }
  };
  const md_name = `/assets/blogs/${which_md}/blog.md`;
  console.log(md_name);
  // 发送请求
  httpRequest.open("GET", md_name, true);
  httpRequest.send();
}

function changePath_pic(text, md_name) {
  let result = text;
  // fix links path
  const reg_get_path = new RegExp(
    "\\[(.*)\\]\\((?!https://|http://)(?:.*)/(.*).md\\)",
    "mg",
  );
  result = result.replace(reg_get_path, "[$1](/blog/$2)");

  // fix pictures path
  const reg_pic = new RegExp(
    "!\\[(.*)\\]\\((?!https://|http://)./(.*)\\)",
    "mg",
  );
  result = result.replace(reg_pic, `![$1](/assets/blogs/${md_name}/$2)`);

  return result;
}
