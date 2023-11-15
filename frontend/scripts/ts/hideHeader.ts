const header = document.getElementsByClassName("mdl-layout__header")[0];
const content = document.getElementsByClassName("mdl-layout__content")[0];
header.classList.add("hiden");
content.addEventListener("scroll", function (event) {
	// console.log('scroll!');
	if (this.scrollTop > 0) {
		header.classList.remove("hiden");
	} else {
		header.classList.add("hiden");
	}
});
