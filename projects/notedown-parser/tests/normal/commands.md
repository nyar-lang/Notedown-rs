

<p>下载地址: <code>https://mirrors.tuna.tsinghua.edu.cn/anaconda/archive/</code></p><p>别乱点直接一路确认, 安装很慢, 可以去看会儿电影</p><p>安装完了就建一个新的隔离环境</p><pre style="background-color:#2b303b;">
<span style="color:#eb6772;">conda</span><span style="color:#abb2bf;"> create</span><span style="color:#eb6772;"> -n</span><span style="color:#abb2bf;"> deeplearning python=3.7
</span><span style="color:#eb6772;">conda</span><span style="color:#abb2bf;"> activate deeplearning</span></pre>
<p>然后添加一些中心包服务器, 比如 <code>conda-forge</code> 是一定要有的, 不然很多包都下不到.</p><pre style="background-color:#2b303b;">
<span style="color:#eb6772;">conda</span><span style="color:#abb2bf;"> config</span><span style="color:#eb6772;"> --add</span><span style="color:#abb2bf;"> channels https://mirrors.tuna.tsinghua.edu.cn/anaconda/pkgs/main/
</span><span style="color:#eb6772;">conda</span><span style="color:#abb2bf;"> config</span><span style="color:#eb6772;"> --add</span><span style="color:#abb2bf;"> channels https://mirrors.tuna.tsinghua.edu.cn/anaconda/cloud/conda-forge/
</span><span style="color:#eb6772;">conda</span><span style="color:#abb2bf;"> config</span><span style="color:#eb6772;"> --add</span><span style="color:#abb2bf;"> channels https://mirrors.tuna.tsinghua.edu.cn/anaconda/cloud/pytorch/
</span><span style="color:#eb6772;">conda</span><span style="color:#abb2bf;"> config</span><span style="color:#eb6772;"> --add</span><span style="color:#abb2bf;"> channels willyd
</span><span style="color:#eb6772;">conda</span><span style="color:#abb2bf;"> config</span><span style="color:#eb6772;"> --set</span><span style="color:#abb2bf;"> show_channel_urls yes
</span><span style="color:#eb6772;">conda</span><span style="color:#abb2bf;"> config</span><span style="color:#eb6772;"> --show</span><span style="color:#abb2bf;"> channels</span></pre>
<p>不要添加那个叫 free 的, 里面都是老的不能用的玩意儿, 如果添加了那最好移除.</p><pre style="background-color:#2b303b;">
<span style="color:#eb6772;">conda</span><span style="color:#abb2bf;"> config</span><span style="color:#eb6772;"> --remove</span><span style="color:#abb2bf;"> channels https://mirrors.tuna.tsinghua.edu.cn/anaconda/pkgs/free/</span></pre>
<p>最后直接一条命令安装即可</p><pre style="background-color:#2b303b;">
<span style="color:#eb6772;">conda</span><span style="color:#abb2bf;"> install tensorflow keras caffe-cpu pytorch-cpu</span><span style="color:#eb6772;"> -y
</span><span style="color:#eb6772;">pip</span><span style="color:#abb2bf;"> install mxnet</span></pre>
<p>千万不要加 <code>-c</code> 除非你住在墙外.</p><p>然后由于 conda 实在是非常非常的慢, 现在可以去睡一觉</p><p>等一觉醒来就可以愉快的使用各大深度学习环境啦</p><p>当然可能由于网络连接问题中断, 那没办法了, 一遍不行就只能再装一遍了</p><hr/><p>最后还可以装一些常用的模型库</p><p>比如 torchvision-cpu gluoncv 啥的</p>
