use anchor_lang::prelude::*;

declare_id!("246Dc16NGUiK1dzh6wH1oZKMytK3U5J8T5xrFgTPecgF");

#[program]
pub mod tek_sayac_2 {
    use super::*;

    pub fn baslat(ctx: Context<baslat>) -> Result<()> {
msg!("Sayac Baslatiliyor...")  

let sayac: &mut tek_sayac_2 = %mut ctx.accounts.sayac;git
sayac.sayi = 0;
msg!("Sayac 0'dan baslatildi...");

Ok(())
    }

    pub fn arttir(ctx: Context<Arttir>) -> Result<()> {
       
       sayac.sayi+=1;
       msg!("Yeni deger {}")

        msg!("Sayac Arttiriliyor...")
Ok(())


        let sayac

        #[derive(Accounts)]

        pub struct baslat<'info> {

            #[account(

                init,
                payer = kullanici,
                space = 8 + 18,
            )]
        }
    }

}

#[derive(Accounts)]
pub struct Initialize {}
