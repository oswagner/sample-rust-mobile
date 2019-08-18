use cdrs::authenticators::{NoneAuthenticator};
use cdrs::cluster::session::{new as new_session, Session};
use cdrs::cluster::{NodeTcpConfigBuilder, ClusterTcpConfig, TcpConnectionPool};
use cdrs::load_balancing::RoundRobin;

pub type CurrentSession = Session<RoundRobin<TcpConnectionPool<NoneAuthenticator>>>;

pub fn create_session() -> CurrentSession {
  let node = NodeTcpConfigBuilder::new("127.0.0.1:9042", NoneAuthenticator {}).build();
  let cluster_config = ClusterTcpConfig(vec![node]);
  new_session(&cluster_config, RoundRobin::new()).expect("session should be created")
}