set -ex

main() {
    local target=
    if [ $TRAVIS_OS_NAME = linux ]; then
        target=x86_64-unknown-linux-musl
        sort=sort
    else
        target=x86_64-apple-darwin
        sort=gsort  # for `sort --sort-version`, from brew's coreutils.
    fi

    # Builds for iOS are done on OSX, but require the specific target to be
    # installed.
    case $TARGET in
        aarch64-apple-ios)
            rustup target install aarch64-apple-ios
            ;;
        armv7-apple-ios)
            rustup target install armv7-apple-ios
            ;;
        armv7s-apple-ios)
            rustup target install armv7s-apple-ios
            ;;
        i386-apple-ios)
            rustup target install i386-apple-ios
            ;;
        x86_64-apple-ios)
            rustup target install x86_64-apple-ios
            ;;
    esac

    # 这段代码是一个Shell脚本命令，用于从指定的Git仓库（在这个例子中是`https://github.com/japaric/cross`）中检索最新的标签（tag），该标签遵循特定的版本命名约定（以`v`开头，后面跟随数字和点）。这个命令主要通过一系列的管道操作（`|`）来实现，每一步的作用如下：

    # 1. **`git ls-remote --tags --refs --exit-code https://github.com/japaric/cross`**:
    #    - `git ls-remote`：列出远程仓库中所有引用（如分支和标签）。
    #    - `--tags`：仅列出标签（tags）。
    #    - `--refs`：只显示引用（即排除任何可能的注解标签对象）。
    #    - `--exit-code`：如果没有找到任何引用，命令将以非零状态退出。这个选项在这里可能不影响输出，但在脚本其他部分可能用于错误检测。
    #    - `https://github.com/japaric/cross`：指定的远程Git仓库URL。

    # 2. **`cut -d/ -f3`**:
    #    - `cut`：用于剪切和分割字符串。
    #    - `-d/`：指定分隔符为`/`。
    #    - `-f3`：选择分割后的第三字段。因为`git ls-remote`的输出格式通常是`<hash>\trefs/tags/<tagname>`，所以这里是为了获取标签名称。

    # 3. **`grep -E '^v[0-9.]+$'`**:
    #    - `grep -E`：使用扩展正则表达式进行搜索。
    #    - `'^v[0-9.]+$'`：正则表达式，匹配以`v`开头，后面跟随数字和点的字符串。这用于筛选遵循特定版本命名约定的标签。

    # 4. **`$sort --version-sort`**:
    #    - `$sort`：这里应该是一个变量替换错误，正常应该是`sort`命令，用于排序输入的行。
    #    - `--version-sort`：按版本号进行排序。这确保了版本号按照预期的方式排序（例如，`v2.10`在`v2.2`之后）。

    # 5. **`tail -n1`**:
    #    - `tail`：输出文件或流的最后部分。
    #    - `-n1`：指定输出最后一行，即最新的版本标签。

    # 整个命令的作用是将远程Git仓库`https://github.com/japaric/cross`的标签列表经过处理和筛选，最终赋值给变量`tag`，其中存储的是遵循`v<数字>.<数字>`格式的最新版本标签。这个命令在自动化脚本中很有用，尤其是在需要自动获取最新软件版本进行下载或构建时。
    # This fetches latest stable release
    local tag=$(git ls-remote --tags --refs --exit-code https://github.com/japaric/cross \
                       | cut -d/ -f3 \
                       | grep -E '^v[0.1.0-9.]+$' \
                       | $sort --version-sort \
                       | tail -n1)

    # 这行命令是使用`curl`下载并执行一个远程Shell脚本，同时通过管道（`|`）将其传递给`sh`命令执行，并向脚本传递了一系列参数。这个过程常用于自动化安装或配置软件。下面是对这个命令各部分的详细解释：

    # 1. **`curl -LSfs https://japaric.github.io/trust/install.sh`**:
    #    - `curl`：是一个工具，用于从服务器传输数据。
    #    - `-L`：如果服务器报告数据在其他地方（例如，重定向），允许curl跟随重定向。
    #    - `-Sfs`：
    #      - `-S`（或`--show-error`）：显示错误。与`-s`一起使用时，仅在发生错误时显示错误。
    #      - `-f`（或`--fail`）：请求失败时（HTTP状态码大于或等于400）直接退出，不输出HTML内容。
    #      - `-s`（或`--silent`）：静默模式。不显示进度条或错误信息。
    #    - `https://japaric.github.io/trust/install.sh`：这是要下载的远程Shell脚本的URL。

    # 2. **`| sh -s --`**:
    #    - `|`：管道符，将前一个命令的输出作为下一个命令的输入。
    #    - `sh`：Shell的一种，用于执行命令。
    #    - `-s`：告诉`sh`从标准输入读取命令。
    #    - `--`：这是一个常见的约定，用于指示命令行参数的结束，确保后续的所有内容都被视为参数，即使它们以`-`开头。

    # 3. **`--force --git japaric/cross --tag $tag --target $target`**:
    #    - `--force`：这通常是告诉脚本忽略某些警告或错误，强制执行操作。
    #    - `--git japaric/cross`：这可能是指定Git仓库的参数。在这里，它指定了`japaric/cross`作为要操作的仓库。
    #    - `--tag $tag`：指定要使用的特定标签，`$tag`是一个变量，它的值应该是之前通过某种方式（比如前一个问题中的脚本）确定的最新版本标签。
    #    - `--target $target`：指定目标平台或配置，`$target`同样是一个变量，其值应该在命令执行前被定义。

    # 整体来看，这个命令是用于自动化下载并执行`https://japaric.github.io/trust/install.sh`脚本的一种方式，同时指定了一些参数来控制脚本的行为，包括使用特定的Git仓库、标签和目标。这种方式常用于CI/CD流程中自动化安装和部署软件。

    curl -LSfs https://japaric.github.io/trust/install.sh | \
        sh -s -- \
           --force \
           --git japaric/cross \
           --tag $tag \
           --target $target
}

main
