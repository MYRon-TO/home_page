<h1>
    <a href="https://github.com/MYRon-TO/yuru" style="font-size: 2.8rem; display: flex; align-items: center;">
    <img src="./assets/images/jellyfish.png" width="70px" height="70px" alt="yuru">
            yuru
    </a>
</h1>

[Here is the source code of my blog](https://github.com/MYRon-TO/yuru)

**These code won't work on windows, I guess :(**

## Quick Start
~~make sure you have installed [**node.js**](https://nodejs.org/en) ,[**mysql**](https://www.mysql.com/downloads/) and [**rust**](https://www.rust-lang.org/zh-CN/tools/install)~~

make sure you have installed [**docker**](https://docs.docker.com/get-docker/) and [**docker-compose**](https://docs.docker.com/compose/install/)

### Run
```bash
mv ./.config/config.toml.example ./.config/config.toml
# 修改配置文件
vim ./.config/config.toml
vim ./db/init_db.sql
# 保证两文件数据库用户名与密码相同
```

```bash
docker compose up --build
```

### Stop
```
<Ctrl-C>
```

## What I use
- ~~apache(or lighttpd)~~
- docker
- mysql/mariadb
- node.js
  - npm
    - ~~typescript~~ <!-- actually, i didn't use it -->
    - marked <!-- markdown parser -->
    - ~~husky~~ <!-- git hook -->
    - material design <!-- css framework -->
        - material design icons
        - material desifn lite
    - sakana-widget <!-- a widget -->
    - matter.js
- fontawesome <!-- icon font -->
- rust
  - cargo
    - axum <!-- web framework -->
    - tokio <!-- async runtime -->
    - tower <!-- service abstraction -->
    - sqlx <!-- database driver -->
    - serde <!-- json parser -->
      - serde_json <!-- json parser -->
      - toml
    - askama <!-- template engine -->
    - walkdir
