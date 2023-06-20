import static com.kms.katalon.core.checkpoint.CheckpointFactory.findCheckpoint
import static com.kms.katalon.core.testcase.TestCaseFactory.findTestCase
import static com.kms.katalon.core.testdata.TestDataFactory.findTestData
import static com.kms.katalon.core.testobject.ObjectRepository.findTestObject
import static com.kms.katalon.core.testobject.ObjectRepository.findWindowsObject
import com.kms.katalon.core.checkpoint.Checkpoint as Checkpoint
import com.kms.katalon.core.cucumber.keyword.CucumberBuiltinKeywords as CucumberKW
import com.kms.katalon.core.mobile.keyword.MobileBuiltInKeywords as Mobile
import com.kms.katalon.core.model.FailureHandling as FailureHandling
import com.kms.katalon.core.testcase.TestCase as TestCase
import com.kms.katalon.core.testdata.TestData as TestData
import com.kms.katalon.core.testng.keyword.TestNGBuiltinKeywords as TestNGKW
import com.kms.katalon.core.testobject.TestObject as TestObject
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import com.kms.katalon.core.webui.keyword.WebUiBuiltInKeywords as WebUI
import com.kms.katalon.core.windows.keyword.WindowsBuiltinKeywords as Windows
import internal.GlobalVariable as GlobalVariable
import org.openqa.selenium.Keys as Keys

WebUI.openBrowser('')

WebUI.navigateToUrl('https://uatbrokerportal.hoxtoninsuranceservices.com/Login')

WebUI.setText(findTestObject('Object Repository/Search-policy/Page_HOXTON/input_Login_UserName'), 'ayazahmad984@gmail.com')

WebUI.setEncryptedText(findTestObject('Object Repository/Search-policy/Page_HOXTON/input_Login_password'), 'r98I3Krbh9FjvLo+XsrGvA==')

WebUI.click(findTestObject('Object Repository/Search-policy/Page_HOXTON/button_Login'))

WebUI.click(findTestObject('Object Repository/Search-policy/Page_HOXTON/span_CREATE A QUOTE OR SEARCH FOR EXISTING _4893e4'))

WebUI.click(findTestObject('Object Repository/Search-policy/Page_HOXTON/a_SEARCH FOR A QUOTE OR POLICY'))

WebUI.click(findTestObject('Object Repository/Search-policy/Page_Search Policy  HOXTON/a_Search'))

WebUI.click(findTestObject('Object Repository/Search-policy/Page_Search Policy  HOXTON/button_WER-P-0001093'))

WebUI.click(findTestObject('Object Repository/Search-policy/Page_HOXTON/a_Jun 2023 - May 2024'))

WebUI.click(findTestObject('Object Repository/Search-policy/Page_HOXTON/td_END-WER-Q-0003006'))

