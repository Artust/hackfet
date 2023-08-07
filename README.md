# HACKFET

## 1.Build & deploy
```bash
./deploy.sh
```

## 2. Get address beneficiary
```bash
near call <address_smart_contract> get_beneficiary --accountId <account_wallet>
```
***

## 3. Change address beneficiary
```bash
near call <address_smart_contract> change_beneficiary '{"beneficiary": "<account receive donate>"}' --accountId <account_wallet>
```

## 4. Donate
```bash
near call <address_smart_contract> send_coin --amount <amount_donate> --accountId <account_wallet>
```

---
**Note:** If would using your account, first login into NEAR using:
```bash
near login
```