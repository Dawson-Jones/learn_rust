use std::time::Duration;

use futures::executor::block_on;

/// 通过 async 标记的语法块会被转换成实现了 Future 特征的状态机。
/// 当阻塞发生时, 同步调用会阻塞当前线程
/// 当Future执行并遇到阻塞时，它会让出当前线程的控制权，这样其它的Future就可以在该线程中运行，这种方式完全不会导致当前线程的阻塞。


async fn hello_world() {
    println!("hello, world!");
}

fn main() {
    // hello_world();  // futures do nothing unless you `.await` or poll them
    // 现在调用就是一个 future 了

    // 一个实现了 Future 的特征对象
    let future/* : impl Future<Output = ()> */ = hello_world();
    block_on(future);


    // 载歌载舞
    block_on(async_main());
}

struct Song {
    author: String,
    name: String,
}

async fn learn_song() -> Song {
    println!("学习中");
    let delay = futures_timer::Delay::new(Duration::from_secs(1));
    delay.await;
    println!("学会啦");

    Song {
        author: "曲婉婷".to_string(),
        name: String::from("《我的歌声里》"),
    }
}

async fn sing_song(song: Song) {
    println!(
        "给大家献上一首{}的{} ~ {}",
        song.author, song.name, "你存在我深深的脑海里~ ~"
    );
}

async fn dance() {
    println!("you can realy dance~ ~");
}

async fn learn_and_sing() {
    let song = learn_song().await;

    // 唱歌必须要在学歌之后
    sing_song(song).await;
}

async fn async_main() {
    let f1 = learn_and_sing();
    let f2 = dance();

    futures::join!(f1, f2);
}