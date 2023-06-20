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

WebUI.navigateToUrl('https://www.ajio.com/')

WebUI.click(findTestObject('Object Repository/Page_Online Shopping for Women, Men, Kids  _34a021/a_INDIE'))

WebUI.click(findTestObject('Object Repository/Page_Indie/input_Kuber Industries_searchVal'))

WebUI.click(findTestObject('Object Repository/Page_Indie/a_shoes'))

WebUI.click(findTestObject('Object Repository/Page_White/img__rilrtl-lazy-img  rilrtl-lazy-img-loaded'))

WebUI.switchToWindowTitle('Buy Black Sports Shoes for Men by PUMA Online | Ajio.com')

WebUI.click(findTestObject('Object Repository/Page_Buy Black Sports Shoes for Men by PUMA_76e13a/span_10'))

WebUI.closeBrowser()

