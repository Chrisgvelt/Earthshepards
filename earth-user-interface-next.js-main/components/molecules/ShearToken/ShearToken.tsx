"use client";
import { useState, useEffect } from "react";
import "@polkadot/api-augment";
import { ApiPromise, WsProvider } from "@polkadot/api";
import type { Balance as Balance2 } from '@polkadot/types/interfaces';

import TokenBox from '../../atoms/TokenBox/TokenBox'
import Container from '../Container/Container'
import Grid from '../Grid/Grid'
import styles from './ShearToken.module.scss'
import Image from 'next/image'

import PluseIcon from './../../../public/plus.svg'

const wsProvider = new WsProvider("wss://www.sheartoken.com/");


interface BalanceType {
  feeFrozen?: string;
  free?: string;
  miscFrozen?: string;
  reserved?: string;
}

const ShearToken = () => {
  const [shearToken_Balance, setShearTokenBalance] = useState<number>(0);
  const [freeBalance, setBalance] = useState<BalanceType>({});
  const [freeBalanceValue, setBalanceValue] = useState<number>(0);


  const boxData = [
    {
      imagePath: 'null',
      title: 'Balance',
      subtitle: '345,890 SHE',
      rate: null,
    }
  ]

  const settings = {
    dots: false,
    infinite: false,
    speed: 500,
    slidesToShow: 3,
  }






  

  return (
    <section className={styles.ShearToken}>
      <Container>
        <Grid>
          <h1 className={styles.title}>Shear Token</h1>
          <div className={styles.boxContainer}>
            {boxData.map((box) => (
              <TokenBox
                imagePath={box.imagePath}
                title={box.title}
                subtitle={shearToken_Balance}
                rate={box.rate}
                key={Number(boxData.indexOf(box))}
              ></TokenBox>
            ))}
            <div className={`${styles.box} ${styles.add}`}>
              <div className="icon">
                <Image src={PluseIcon} alt="add-icon"></Image>
              </div>
            </div>
          </div>
        </Grid>
      </Container>
    </section>
  )
}

export default ShearToken
