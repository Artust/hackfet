# HACKFET

---

## Description

### Task

- Id
- Link
- Description
- Owner
- Developer
  - AccountId
  - Approve
  - IsBid
  - Process:
    1. To do
    2. In progress
    3. Wait review
    4. Done
- Inspector
  - AccountId
  - Verify:
    1. None
    2. Reject
    3. Pass
  - Comment
- Investment (tiền đầu tư)
  - Inspection
  - Development
- Contract bond (tiền bảo lãnh thực hiện hợp đồng)
- Status: các step của task
  1. Tạo mới
  2. Assign dev
  3. Dev đặt cọc
  4. Inspector đồng ý
  5. Thanh toán (Development + Contract bond)
- Greylist: [AccountId]

### Role

1. Root account
   - Add/Remove Admin
   - Add/Remove Inspector
   - Edit task
2. Admin
   - Edit task
3. Investor (nhà đầu tư)
   - Create
   - Edit task (chưa có ai tham gia vào task)
4. Inspector
   - Edit task
5. Developer

### Flow

1. Create task
   - Create, Update: deposit, withdrawal
   - Read
   - Delete: withdrawal
2. Developer pick task
   - Assign: deposit
   - Drop out: withdrawal
3. Owner approve Developer
   - Update task
   - Reject Developer: update to Greylist
4. Inspector verify task
   - Reject:
     - change Developer status to In progress
     - add comment
   - Done
     - change Developer status to Done
     - Pay for Inspector equal Investment.Inspection
5. Pay for dev
   - Transfer to Developer total amount (include Investment.Inspection + Contract bond)

---

## Building & run test

### 1.Build & deploy

```bash
./deploy.sh
```

### 2. Get address beneficiary

```bash
near call <address_smart_contract> get_beneficiary --accountId <account_wallet>
```

### 3. Change address beneficiary

```bash
near call <address_smart_contract> change_beneficiary '{"beneficiary": "<account receive donate>"}' --accountId <account_wallet>
```

### 4. Donate

```bash
near call <address_smart_contract> send_coin --amount <amount_donate> --accountId <account_wallet>
```

**Note:** If would using your account, first login into NEAR using:

```bash
near login
```
