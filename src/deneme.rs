use soroban_sdk::{contractimpl, Address, Env, Map};

pub struct TokenContract;

#[contractimpl]
impl TokenContract {
    // Yeni bir Map ekliyoruz, bu harita hesap adreslerini ve dondurulmuş durumlarını takip edecek.
    pub fn freeze_account(env: Env, account: Address) {
        // 'frozen_accounts' Map'ini al veya yeni bir Map oluştur
        let mut frozen_accounts: Map<Address, bool> = env.storage().persistent().get(&"frozen_accounts").unwrap_or(Map::new(&env));

        // Hesabı dondur
        frozen_accounts.set(account.clone(), true);

        // Güncellenen Map'i sakla
        env.storage().persistent().set(&"frozen_accounts", &frozen_accounts);
    }

    pub fn unfreeze_account(env: Env, account: Address) {
        // 'frozen_accounts' Map'ini al veya yeni bir Map oluştur
        let mut frozen_accounts: Map<Address, bool> = env.storage().persistent().get(&"frozen_accounts").unwrap_or(Map::new(&env));

        // Hesabı çöz
        frozen_accounts.remove(account);

        // Güncellenen Map'i sakla
        env.storage().persistent().set(&"frozen_accounts", &frozen_accounts);
    }

    pub fn transfer(env: Env, from: Address, to: Address, amount: i128) {
        // 'frozen_accounts' Map'ini al veya yeni bir Map oluştur
        let frozen_accounts: Map<Address, bool> = env.storage().persistent().get(&"frozen_accounts").unwrap_or(Map::new(&env));

        // Eğer 'from' hesabı dondurulmuşsa transferi reddet
        if frozen_accounts.get(from).unwrap_or(false) {
            panic!("Hesap dondurulmuş, transfer gerçekleştirilemez.");
        }

        // Burada transfer işlemi gerçekleştirilir.
        // Bu kısım, mevcut transfer fonksiyonunun mantığına göre doldurulmalıdır.
    }
}
