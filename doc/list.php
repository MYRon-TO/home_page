<!DOCTYPE html>
<html>

<head>

  <!-- meta -->
  <meta charset="utf-8">
  <meta name="viewport" content="width=device-width, initial-scale=1.0">
  <title>A New Page</title>

  <!-- material design lite -->
  <link rel="stylesheet" href="node_modules/material-design-lite/material.min.css">
  <link rel="stylesheet" href="node_modules/@material-design-icons/font/index.css">
  <script src="node_modules/material-design-lite/material.min.js"></script>

  <!-- css -->
  <link rel="stylesheet" href="styles/header.css">
  <link rel="stylesheet" href="styles/card.css">

  <style>
  </style>

</head>

<body>

  <div class="mdl-layout mdl-js-layout mdl-layout--fixed-header mdl-layout--no-desktop-drawer-button">

    <!-- # header -->
    <!-- <header class="mdl-layout__header mdl-layout__header--waterfall mdl-layout__header--waterfall-hide-top"> -->
    <header class="mdl-layout__header mdl-layout__header--waterfall">

      <!-- ## title -->
      <div class="mdl-layout__header-row">
        <span class="mdl-layout-title">博客</span>
        <div class="mdl-layout-spacer"></div>

        <!-- this will hide in small screen -->
        <nav class="mdl-navigation mdl-layout--large-screen-only">
          <a class="mdl-navigation__link" href="index.html">首页</a>
          <a class="mdl-navigation__link" href="blog_list.html">博客</a>
          <a class="mdl-navigation__link" href="friend.html">友链</a>
          <a class="mdl-navigation__link" href="about.html">关于</a>
        </nav>
      </div>

    </header>

    <!-- ## link -->
    <!-- this will hide in large screen -->
    <div class="mdl-layout__drawer mdl-layout--small-screen-only">
      <div class="mdl-layout-title">
        传送门
      </div>
      <nav class="mdl-navigation">
        <a class="mdl-navigation__link" href="index.html">首页</a>
        <a class="mdl-navigation__link" href="blog_list.html">博客</a>
        <a class="mdl-navigation__link" href="friend.html">友链</a>
        <a class="mdl-navigation__link" href="about.html">关于</a>
      </nav>
    </div>

    <main style="
      padding-top: 0;
      " class="mdl-layout__content" id="content">
      <!-- # content -->
      <!-- ### grid -->
      <div class="mdl-grid grid_card">

<?php
$con = mysql_connect("localhost","BlueBird");
if (!$con){
die('Could not connect: ' . mysql_error());
}

mysql_close($con);
?>

      </div>

      <!-- ## footer -->
      <footer class="mdl-mini-footer" style="display: flex;">
        <div class="mdl-mini-footer__left-section">
          <div class="mdl-logo">Title</div>
          <ul class="mdl-mini-footer__link-list">
            <li><a href="https://www.njupt.edu.cn">NJUPT</a></li>
            <li><a href="https://www.github.com">github</a></li>
          </ul>
        </div>
      </footer>

    </main>
    <div class="mdl-layout__obfuscator"></div>
  </div>

</body>

</html>
