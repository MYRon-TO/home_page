<?php
  declare(strict_types=1); // 开启严格模式
?>
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

  use Yosymfony\Toml\Toml;
  require __DIR__ . '/vendor/autoload.php';

  // # Result and Status
  enum Status{
  // ## Status
  // Ok: 成功
  // Err: 失败
    case Ok;
    case Err;
  };
  class Result{
  // ## Result
  // status: 状态
  // msg: 消息
  // isOk(): 是否成功
  // unwarp(): 获取消息, 如果失败则抛出异常
    public Status $status;
    public mixed $msg;
    public function __construct(Status $status, mixed $msg){
      $this->status = $status;
      $this->msg = $msg;
    }
    public function isOk(): bool{
      return $this->status == Status::Ok;
    }
    public function unwarp(): mixed{
      if($this->isOk()){
        return $this->msg;
      }else{
        throw new Exception("Failed to unwarp Result: " . (is_string($this->msg) ? $this->msg : "unknown error : msg is not string"));
      }
    }
  }

  class DBconnection{
  // ## DBconnection
  // con: mysqli连接
  // query(sql): 执行sql语句, 返回Result
    public mysqli $con;
    public function __construct(string $host, string $user, string $password){
      $con = mysqli_connect($host, $user, $password);
      if(mysqli_connect_errno()){
        throw new Exception("Failed to connect to MySQL: " . (mysqli_connect_error() ?? "unknown error : msg is not string"));
      }
      $this->con = $con;
    }

    public function query(string $sql): Result{
      $result = $this->con->query($sql);
      if($result){
        return new Result(Status::Ok, $result);
      }else{
        return new Result(Status::Err, "Failed to query: " . $this->con->error);
      }
    }
    
    public function checkDB(): Result{
      $result = $this->query("SHOW DATABASES LIKE 'blog'");
      if($result->isOk()){
        if($result->unwarp()->num_rows == 0){
          return new Result(Status::Err, "Database 'blog' does not exist");
        }else{
          return new Result(Status::Ok, "Database 'blog' exists");
        }
      }else{
        return new Result(Status::Err, $result->unwarp());
      }
    }

    function __destruct(){
      mysqli_close($this->con);
    }
  }

  function getConfig(): Result{
  // ## getConfig
  // 从配置文件中获取数据库配置
    $config   = Toml::ParseFile('../.config/config.toml');
    $host     = $config['database']['host'] ?? "localhost";   // string
    $user     = $config['database']['user'] ?? "user";        // string
    $password = $config['database']['password'] ?? "";        // string

    if (settype($host, "string")&&settype($user, "string")&&settype($password, "string")){
      return new Result(Status::Ok, [$host, $user, $password]);
    }else{
      return new Result(Status::Err, "Failed to settype in getConfig");
    }
  }

  function getDBconnection(): Result{
  // ## getDBconnection
  // 获取数据库连接
    $config = getConfig();
    if($config->isOk()){
      try{
        $host     = $config->unwarp()[0];
        $user     = $config->unwarp()[1];
        $password = $config->unwarp()[2];
        return new Result(Status::Ok, new DBconnection($host, $user, $password));
      }catch(Exception $e){
        return new Result(Status::Err, $e->getMessage());
      }
    }else{
      return new Result(Status::Err, $config->unwarp());
    }
  }

  function main():void{
    $con = getDBconnection()->unwarp();// 获取数据库连接
  }

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
