#[cfg(feature = "std")]
use serde::{Deserialize, Serialize};
use codec::{Encode, Decode};
use sp_std::vec::Vec;
use sp_debug_derive::RuntimeDebug;

///虚拟机商品信息
#[derive(Encode, Decode, RuntimeDebug,Clone, Eq, PartialEq, Default)]
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
pub struct VirtualMachine<AccountId,BlockNumber> {
	/// 虚机信息id
	pub id: Vec<u8>,
	pub account_id: AccountId,
	///公网ip
	pub server_ipv4: Vec<u8>,
	///端口号
	pub server_port: u32,
	///CPU数量
	pub cpu_cores: u32,
	///内存大小
	#[cfg_attr(feature = "std", serde(serialize_with = "string_serialize"))]
	pub ram_size: u128,
	///磁盘大小GB
	#[cfg_attr(feature = "std", serde(serialize_with = "string_serialize"))]
	pub disk_size: u128,
	///带宽
	pub bandwidth: u32,
	///服务时长(天数)
	pub server_period: u32,
	/// 虚机价格
	#[cfg_attr(feature = "std", serde(serialize_with = "string_serialize"))]
	pub price: u128,
	///操作系统
	pub operating_system: Vec<u8>,
	///创建人
	pub create_user_id: Vec<u8>,
	///创建时间(区块高度)
	pub block_number: BlockNumber

}

impl<AccountId,BlockNumber> VirtualMachine<AccountId,BlockNumber> {
	pub fn new (id: Vec<u8>,account_id:AccountId,server_ipv4: Vec<u8>,server_port: u32,cpu_cores: u32,ram_size: u128,disk_size: u128,bandwidth: u32,server_period: u32,
				price: u128,operating_system: Vec<u8>,create_user_id: Vec<u8>,block_number: BlockNumber) -> Self {
		VirtualMachine {
			id,
			account_id,
			server_ipv4,
			server_port,
			cpu_cores,
			ram_size,
			disk_size,
			bandwidth,
			server_period,
			price,
			operating_system,
			create_user_id,
			block_number
		}
	}
}

#[derive(Encode, Decode, RuntimeDebug, Clone, Eq, PartialEq, Default)]
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
pub struct VirtualMachineList<AccountId,BlockNumber> {
	/// 内容
	pub content: Vec<VirtualMachine<AccountId,BlockNumber>>
}

impl<AccountId,BlockNumber> VirtualMachineList<AccountId,BlockNumber> {
	pub fn new (content: Vec<VirtualMachine<AccountId,BlockNumber>>) -> Self {
		VirtualMachineList {
			content
		}
	}
}


// u128 does not serialize well into JSON for `handlebars`, so we represent it as a string.
#[cfg(feature = "std")]
fn string_serialize<S>(x: &u128, s: S) -> Result<S::Ok, S::Error> where
	S: serde::Serializer
{
	s.serialize_str(&x.to_string())
}
