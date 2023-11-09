const text = `
- 终点(目的站、信宿): 接收数据的设备 ![消息与信道](https://www.njupt.edu.cn)
![数据通信系统的模型](./src/数据通信系统的模型.png)
[消息与信道](./消息与信道.md)
- 终点(目的站、信宿): 接收数据的设备 [消息与信道](./test/消息与信道.md)
- 终点(目的站、信宿): 接收数据的设备 [消息与信道](https://www.njupt.edu.cn)
`;


// const reg_get_path = new RegExp("!\\[(.*)\\]\\((?!https://\|http://)(?:.*)/(.*)\\)", "mg");
// const result_get_path = text.replace(reg_get_path, "![$1](assets/blogs/src/$3)");
// const result_get_path = reg_get_path.exec(text);
//
	const reg_get_path = new RegExp(
		// "(?:[.*])(?!https://|http://)(.*/)(.*).md",
		"\\[(.*)\\]\\((?!https://|http://)(?:.*)/(.*).md\\)",
		"mg",
	);
	const result_change_path = text.replace(
		reg_get_path,
		"[$1](blog.html?markdown=$2)",
	);

// const result_get_path = text.replace(reg_get_path, "assets/blogs/$1$2");

console.log(result_change_path);
