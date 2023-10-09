import React from 'react';
import { createDrawerNavigator } from '@react-navigation/drawer';
import { NavigationContainer } from '@react-navigation/native';

import HomeScreen from '../components/HomeScreen';
import AboutScreen from '../components/AboutScreen';
import VaraScreen from '../components/VaraScreen';
import SettingsScreen from '../components/SettingsScreen';
import ContactScreen from '../components/ContactScreen';
import WalletsScreen from '../components/WalletScreen';
import BetsScreen from '../components/BetsScreen';
import LotteryScreen from '../components/LotteryScreen';
import GamesScreen from '../components/GamesScreen';
import GetVaraScreen from '../components/ChangeNowScreen';

const Drawer = createDrawerNavigator();

const DrawerNavigator = () => {
  return (
    <NavigationContainer>
      <Drawer.Navigator initialRouteName="Home">
        <Drawer.Screen name="Home" component={HomeScreen} />
        <Drawer.Screen name="Wallet" component={WalletsScreen} />
        <Drawer.Screen name="Get Vara" component={GetVaraScreen} />
        <Drawer.Screen name="Vara Bets" component={BetsScreen} />
        <Drawer.Screen name="Miracle Lottery" component={LotteryScreen} />
        <Drawer.Screen name="Vara Network" component={VaraScreen} />
        <Drawer.Screen name="Settings" component={SettingsScreen} />
        <Drawer.Screen name="OnChain Games" component={GamesScreen} />
        <Drawer.Screen name="About" component={AboutScreen} />
        <Drawer.Screen name="Join the Community." component={ContactScreen} />
      </Drawer.Navigator>
    </NavigationContainer>
  );
};

export default DrawerNavigator;